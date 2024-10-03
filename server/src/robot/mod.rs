pub mod robot_state;
pub mod constants;

use robot_state::{limit_angle, shortest_angle_diff, Coord4DOF, JointState, RobotState};
use constants::*;
use std::{f64::consts::PI, sync::Arc};
use tokio::time::{sleep, Instant, Duration};

use axum::{routing::get, Router};
use socketioxide::{
    extract::{Data, SocketRef, State},
    SocketIo,
};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

const BROADCAST_PERIOD_MS: u64 = 20;
const CONTROLLER_LOOP_TIME_MS: u64 = 5;
const CONTROLLER_LOOP_TIME_S: f64 = CONTROLLER_LOOP_TIME_MS as f64/1000.0;

/// 
const MAX_BASE_LINEAR_VEL: f64 = 0.06;
const BASE_LINEAR_P: f64 = 1.0;
const BASE_LINEAR_D: f64 = 0.5;

const MAX_BASE_ANGLE_VEL: f64 = 3.0;
const BASE_ANGLE_P: f64 = 0.5;
const BASE_ANGLE_D: f64 = 0.1;

/// Max Angular velcoity (deg/sec)
const MAX_ANGULAR_VELOCITY: f64 = 18.0;
/// Max Angular acceleration (deg/sec^2)
const MAX_ANGULAR_ACCELERATION: f64 = 9.0;

const ANGLE_P: f64 = 0.7;
const ANGLE_D: f64 = 1.5;

const FEEDFORWARD_FACTOR: f64 = 2.22;

/// Max linear acceleration (mm/sec^2)
const MAX_LINEAR_ACCELERATION: f64 = 40.0;
/// Max linear velocity (mm/sec)
const MAX_LINEAR_VELOCITY: f64 = 80.0;

const LINEAR_P: f64 = 2.5;
const LINEAR_D: f64 = 4.0;


fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn radians_to_degrees(degrees: f64) -> f64 {
    degrees * 180.0 / PI
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on(
        "set joint state",
        |Data::<JointState>(data), robot_lock: State<RobotLock>| async move {
            
            {
                robot_lock.write().await.set_joint_target_state(data, true);
            }
        },
    );

    socket.on(
        "set coord state",
        |Data::<Coord4DOF>(data), robot_lock: State<RobotLock>| async move {
            
            {
                robot_lock.write().await.set_target_coord_state(data);
            }
        },
    );

    socket.on(
        "set base state",
        |Data::<Coord4DOF>(data), robot_lock: State<RobotLock>| async move {
            {
                robot_lock.write().await.set_target_base_state(data);
            }
        },
    );


    socket.on_disconnect(|| async move {
        info!("Client disconnected");
    });
}

pub type RobotLock = Arc<RwLock<Robot>>;
pub struct Robot {
    /// The current state of the robot.
    state: RobotState,
    /// The robot's target state that the controller will work to get to.
    target_state: RobotState,
    /// The coordinate provided by the user that the end effector should reach.
    target_coord_state: Option<Coord4DOF>,
    /// The robot's velocity.
    velocity: RobotState,
}

impl Robot {
    pub async fn new() -> Arc<RwLock<Self>> {
        let robot_lock: RobotLock = Arc::new(RwLock::new(Self { state: RobotState::default(), target_state: RobotState::default(), target_coord_state: None, velocity: RobotState::default()}));
        
        // Enables logging
        tracing::subscriber::set_global_default(FmtSubscriber::default()).expect("Unable to enable logging");
        
        // Create websocket.
        let (layer, io) = SocketIo::builder().with_state(robot_lock.clone()).build_layer();

        io.ns("/", on_connect);

        let io_handler: Arc<RwLock<SocketIo>> = Arc::new(RwLock::new(io));

        let app: Router = axum::Router::new()
            .route("/", get(|| async { "Robot Server" }))
            .with_state(io_handler.clone())
            .layer(
                ServiceBuilder::new()
                    .layer(CorsLayer::permissive())
                    .layer(layer),
            );

            // Start the controller and broadcasting state messages to client's.
            Self::controller(robot_lock.clone());

            Self::broadcast(robot_lock.clone(), io_handler.clone());

            axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
                .serve(app.into_make_service())
                .await.expect("Could not start websocket");

            robot_lock
        }

    /// Starts a thread that works to broadcast the state of the robot to client's.
    fn broadcast(robot_lock: RobotLock, io_handler: Arc<RwLock<SocketIo>>) {
        tokio::spawn(async move {
            loop {
                let start = Instant::now();
                
                let state;
                let coords;
                {
                    let robot = robot_lock.read().await;
                    state = robot.get_state();
                    coords = robot.get_coord_state();
                }

                // This is bad. Fix this.
                {
                    let socket = io_handler.read().await;
                    let _ = socket.emit("joint state", state);
                    let _ = socket.emit("base coords", coords);
                }

                // Sleep to keep the loop operating at the specified frequency.
                let loop_duration = Instant::now().duration_since(start);
                if let Some(sleep_duration) = Duration::from_millis(BROADCAST_PERIOD_MS).checked_sub(loop_duration) {
                    sleep(sleep_duration).await;
                }
                
            }
        });
    }

    /// Starts a thread to simulate the robot's change in state as it tries to reach the provided targets.
    fn controller(robot_lock: RobotLock){
        tokio::spawn(async move {
            loop {
                let start = Instant::now();
                
                // If a target coordinate state exists perform ik to calculate the required joint target.
                {
                    let target_coord_state = robot_lock.read().await.target_coord_state;
                    if let Some(coord_state) = target_coord_state {
                        let found_state = robot_lock.write().await.ik(coord_state, true).is_some();
                        // If feedforwad ik cannot find a solution try without as it may cause it to command an out of reach position.
                        if !found_state {robot_lock.write().await.ik(coord_state, false);}
                    }
                }

                // Collect values from the robot after ik.
                let joint_state;
                let joint_target;
                let base_state;
                let base_target;
                let veloctiy;
                {
                    let robot = robot_lock.read().await;
                    joint_state = robot.state.joint_state;
                    joint_target = robot.target_state.joint_state;

                    base_state = robot.state.base_state;
                    base_target = robot.target_state.base_state;
                    veloctiy = robot.velocity;
                }
                
                // Perform controller calcualtions for base motion. Find the error and feed it into the PD controller for velocity.
                let mut base_state_error = Coord4DOF::default();
                base_state_error.x = base_target.x - base_state.x;
                base_state_error.y = base_target.y - base_state.y;
                base_state_error.z = base_target.z - base_state.z;
                base_state_error.theta = shortest_angle_diff(base_target.theta, base_state.theta);
                
                let mut base_velocity = base_state_error.apply_control(BASE_LINEAR_P, BASE_ANGLE_P);
                base_velocity = base_velocity - base_velocity.apply_control(BASE_LINEAR_D, BASE_ANGLE_D);
                
                base_velocity.clamp(MAX_BASE_LINEAR_VEL, MAX_BASE_ANGLE_VEL);
                
                // Update base state with velocity.
                let new_base_state = base_state + base_velocity.val_mul(CONTROLLER_LOOP_TIME_S);
                
                
                // Perform controller calcualtions for joint motion. Find the error and feed it into the PD controller for acceleration.
                let joint_state_error = JointState::clamped_sub(joint_target, joint_state);
                
                let mut joint_state_velocity: JointState = veloctiy.joint_state;
                let mut joint_state_acceleration = JointState::default();

                // Calculate P.
                joint_state_acceleration.swing_rotation_deg = joint_state_error.swing_rotation_deg*ANGLE_P;
                joint_state_acceleration.lift_elevation_mm = joint_state_error.lift_elevation_mm*LINEAR_P;
                joint_state_acceleration.elbow_rotation_deg = joint_state_error.elbow_rotation_deg*ANGLE_P;
                joint_state_acceleration.wrist_rotation_deg = joint_state_error.wrist_rotation_deg*ANGLE_P;
                joint_state_acceleration.gripper_open_mm = joint_state_error.gripper_open_mm*LINEAR_P;

                // Caculate D.
                joint_state_acceleration.swing_rotation_deg += -joint_state_velocity.swing_rotation_deg*ANGLE_D;
                joint_state_acceleration.lift_elevation_mm += -joint_state_velocity.lift_elevation_mm*LINEAR_D;
                joint_state_acceleration.elbow_rotation_deg += -joint_state_velocity.elbow_rotation_deg*ANGLE_D;
                joint_state_acceleration.wrist_rotation_deg += -joint_state_velocity.wrist_rotation_deg*ANGLE_D;
                joint_state_acceleration.gripper_open_mm += -joint_state_velocity.gripper_open_mm*LINEAR_D;

                // Clamp acceleration within the max. The max acceleration is inversely scaled by the length of the arms to allow the end effector to be moved equally by all joints.
                joint_state_acceleration.swing_rotation_deg = joint_state_acceleration.swing_rotation_deg.clamp(-MAX_ANGULAR_ACCELERATION/ELBOW_LENGTH_M, MAX_ANGULAR_ACCELERATION/ELBOW_LENGTH_M);
                joint_state_acceleration.lift_elevation_mm = joint_state_acceleration.lift_elevation_mm.clamp(-MAX_LINEAR_ACCELERATION, MAX_LINEAR_ACCELERATION);
                joint_state_acceleration.elbow_rotation_deg = joint_state_acceleration.elbow_rotation_deg.clamp(-MAX_ANGULAR_ACCELERATION, MAX_ANGULAR_ACCELERATION);
                joint_state_acceleration.wrist_rotation_deg = joint_state_acceleration.wrist_rotation_deg.clamp(-MAX_ANGULAR_ACCELERATION/GRIPPER_LENGTH_M, MAX_ANGULAR_ACCELERATION/GRIPPER_LENGTH_M);
                joint_state_acceleration.gripper_open_mm = joint_state_acceleration.gripper_open_mm.clamp(-MAX_LINEAR_ACCELERATION, MAX_LINEAR_ACCELERATION);

                // Apply acceleration to update the velocity.
                joint_state_velocity = joint_state_velocity + joint_state_acceleration.val_mul(CONTROLLER_LOOP_TIME_S);

                // Clamp velocity within the max. The max acceleration is inversely scaled by the length of the arms to allow the end effector to be moved equally by all joints.
                joint_state_velocity.swing_rotation_deg = joint_state_velocity.swing_rotation_deg.clamp(-MAX_ANGULAR_VELOCITY/ELBOW_LENGTH_M, MAX_ANGULAR_VELOCITY/ELBOW_LENGTH_M);
                joint_state_velocity.lift_elevation_mm = joint_state_velocity.lift_elevation_mm.clamp(-MAX_LINEAR_VELOCITY, MAX_LINEAR_VELOCITY);
                joint_state_velocity.elbow_rotation_deg = joint_state_velocity.elbow_rotation_deg.clamp(-MAX_ANGULAR_VELOCITY, MAX_ANGULAR_VELOCITY);
                joint_state_velocity.wrist_rotation_deg = joint_state_velocity.wrist_rotation_deg.clamp(-MAX_ANGULAR_VELOCITY/GRIPPER_LENGTH_M, MAX_ANGULAR_VELOCITY/GRIPPER_LENGTH_M);
                joint_state_velocity.gripper_open_mm = joint_state_velocity.gripper_open_mm.clamp(-MAX_LINEAR_VELOCITY, MAX_LINEAR_VELOCITY);

                // Update by applying velocity to the current state and storing the velocity of the joints and base.
                {
                    let mut robot = robot_lock.write().await;
                    robot.set_state(joint_state+joint_state_velocity.val_mul(CONTROLLER_LOOP_TIME_S), new_base_state);
                    robot.velocity.joint_state = joint_state_velocity;
                    robot.velocity.base_state = base_velocity;
                }

                // Sleep to keep the loop operating at the specified frequency.
                let loop_duration = Instant::now().duration_since(start);
                if let Some(sleep_duration) = Duration::from_millis(CONTROLLER_LOOP_TIME_MS).checked_sub(loop_duration) {
                    sleep(sleep_duration).await;
                };
            }
        });

    }

    // Set the state of the robot's joints and base.
    fn set_state(&mut self, mut new_joint_state: JointState, new_base_state: Coord4DOF) {
        new_joint_state.check_limits();
        
        self.state.joint_state = new_joint_state;
        self.state.base_state = new_base_state;
    }

    // Retyurns the state of the robot.
    pub fn get_state(&self) -> RobotState {
        return self.state;
    }

    /// Sets the target state for all the joints of the robot. If `erase_coord_target` is true the current `target_coord_state` is erased to stop ik calcualtions.
    pub fn set_joint_target_state(&mut self, mut target_state: JointState, erase_coord_target: bool) {
        target_state.check_limits();
        self.target_state.joint_state = target_state;

        if erase_coord_target {
            // Is joint state is set refresh target state (I do not like this, it should only be wehn called externally)
            self.target_coord_state = None;
        }
    }

    /// Performs inverse kinematics using the current base position and target end effector state to return a joint state that will reach the target.
    /// Applys a feedforward approach to the position of the joints to counter the motion of the base if `apply_feedforward` is true.
    fn ik(&mut self, coord_state: Coord4DOF, apply_feedforward: bool) -> Option<JointState> {

        let mut target_state: JointState = JointState::default();

        // Get the radian andle of the end effector. Apply the angular velocity of base to counter its rotation.
        let end_effector_rad = degrees_to_radians(limit_angle(coord_state.theta - self.velocity.base_state.theta*FEEDFORWARD_FACTOR));

        let end_effector_to_base;
        if apply_feedforward {
            // Get the radian andle of the end effector. Apply the angular velocity of base to counter its rotation.
            let end_effector_rad = degrees_to_radians(limit_angle(coord_state.theta - self.velocity.base_state.theta*FEEDFORWARD_FACTOR));

            // Calculate the velocity applied to end effector due to the rotation of the base and its linear motion.
            let current_state = self.get_coord_state();
            let base_applied_x_vel = self.velocity.base_state.x - current_state.y*degrees_to_radians(self.velocity.base_state.theta);
            let base_applied_y_vel = self.velocity.base_state.y + current_state.x*degrees_to_radians(self.velocity.base_state.theta);

            // Get the position of the end effectors base that the wrist and elbow must be positioned to meet the end effector.
            // Apply the feedforward of the bases velocity in the xyz to counter the base's motion.
            end_effector_to_base =  Coord4DOF{
                x: coord_state.x - self.state.base_state.x - FEEDFORWARD_FACTOR*base_applied_x_vel - GRIPPER_LENGTH_M*(end_effector_rad.cos()),
                y: coord_state.y - self.state.base_state.y - FEEDFORWARD_FACTOR*base_applied_y_vel - GRIPPER_LENGTH_M*(end_effector_rad.sin()),
                z: coord_state.z - self.state.base_state.z - FEEDFORWARD_FACTOR*self.velocity.base_state.z,
                theta: coord_state.theta
            };
        } else {
            end_effector_to_base =  Coord4DOF{
                x: coord_state.x - self.state.base_state.x  - GRIPPER_LENGTH_M*(end_effector_rad.cos()),
                y: coord_state.y - self.state.base_state.y  - GRIPPER_LENGTH_M*(end_effector_rad.sin()),
                z: coord_state.z - self.state.base_state.z,
                theta: coord_state.theta
            };
        }

        // Using cosine law to calculate the angles required by the swing and elbow to meet the end effector.
        let base_angle = (end_effector_to_base.y).atan2(end_effector_to_base.x);

        let c = (end_effector_to_base.x.powf(2.0) + end_effector_to_base.y.powf(2.0)).sqrt();
        if c > WRIST_LENGTH_M+ELBOW_LENGTH_M {return None;}

        let elbow_angle: f64 = -(PI - ((c.powf(2.0) - ELBOW_LENGTH_M.powf(2.0) - WRIST_LENGTH_M.powf(2.0))/(-2.0*ELBOW_LENGTH_M*WRIST_LENGTH_M)).acos());
        
        let swing_angle_local = ((WRIST_LENGTH_M.powf(2.0) - ELBOW_LENGTH_M.powf(2.0) - c.powf(2.0))/(-2.0*ELBOW_LENGTH_M*c)).acos();
        
        // If no solution is found return None;
        if elbow_angle.is_nan() || swing_angle_local.is_nan() {return None;}

        let swing_angle = base_angle + swing_angle_local;
        
        // Apply the calculated states.
        target_state.swing_rotation_deg = radians_to_degrees(swing_angle) - self.state.base_state.theta;
        target_state.elbow_rotation_deg = radians_to_degrees(elbow_angle);
        target_state.wrist_rotation_deg = radians_to_degrees(end_effector_rad - elbow_angle - swing_angle);
        target_state.lift_elevation_mm = end_effector_to_base.z * 1000.0;

        self.set_joint_target_state(target_state, false);
        
        return Some(target_state);
    }

    pub fn set_target_coord_state(&mut self, coord_state: Coord4DOF) {
        self.target_coord_state = Some(coord_state);
    }

    pub fn set_target_base_state(&mut self, coord_state: Coord4DOF) {
        self.target_state.base_state = coord_state;
    }

    /// Get the end effectors current position in space.
    pub fn get_coord_state(&self) -> Coord4DOF {
        let joint_state = self.state.joint_state;
        let base_state = self.state.base_state;
        
        let mut coords = Coord4DOF::default();
        coords.x = base_state.x;
        coords.y = base_state.y;
        coords.z = joint_state.lift_elevation_mm/1000.0 + base_state.z;

        let elbow_angle_rad = degrees_to_radians(base_state.theta+ joint_state.swing_rotation_deg);
        let wrist_angle_rad = elbow_angle_rad+degrees_to_radians(joint_state.elbow_rotation_deg);
        let gripper_angle_rad = wrist_angle_rad + degrees_to_radians(joint_state.wrist_rotation_deg);

        // Calculate elbow coordinates
        coords.x += ELBOW_LENGTH_M * elbow_angle_rad.cos();
        coords.y += ELBOW_LENGTH_M * elbow_angle_rad.sin();

        // Calculate wrist coordinates relative to the elbow
        coords.x += WRIST_LENGTH_M * wrist_angle_rad.cos();
        coords.y += WRIST_LENGTH_M * wrist_angle_rad.sin();

        // Calculate gripper coordinates relative to the wrist
        coords.x += GRIPPER_LENGTH_M * gripper_angle_rad.cos();
        coords.y += GRIPPER_LENGTH_M * gripper_angle_rad.sin();

        coords.theta = radians_to_degrees(gripper_angle_rad);

        return coords;
    }
}