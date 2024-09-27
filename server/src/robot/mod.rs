pub mod robot_state;
pub mod constants;

use robot_state::{Coord4DOF, clamp_angle_diff, Coord2D, RobotJointState};
use constants::*;
use tokio::sync::RwLock;
use std::{f64::consts::PI, sync::Arc};
use tokio::time::{sleep, Instant, Duration};

const CONTROLLER_LOOP_TIME_MS: u64 = 20;
const CONTROLLER_LOOP_TIME_S: f64 = CONTROLLER_LOOP_TIME_MS as f64/1000.0;

/// Max Angular velcoity (deg/sec)
const MAX_ANGULAR_VELOCITY: f64 = 18.0;
/// Max linear velocity (mm/sec)
const MAX_LINEAR_VELOCITY: f64 = 80.0;

/// Max Angular acceleration (deg/sec^2)
const MAX_ANGULAR_ACCELERATION: f64 = 5.0;
/// Max linear acceleration (mm/sec^2)
const MAX_LINEAR_ACCELERATION: f64 = 40.0;

const ANGLE_P: f64 = 0.23;
const LINEAR_P: f64 = 1.35;

const ANGLE_D: f64 = 0.9;
const LINEAR_D: f64 = 3.3;


pub type RobotLock = Arc<RwLock<Robot>>;

struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64,) -> Self {
        Circle { x: x, y: y, r: r }
    }
}

pub struct Robot {
    state: RobotJointState,
    target_state: RobotJointState,
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn radians_to_degrees(degrees: f64) -> f64 {
    degrees * 180.0 / PI
}

impl Robot {
    pub fn new() -> Arc<RwLock<Self>> {
        let robot_lock: RobotLock = Arc::new(RwLock::new(Self { state: RobotJointState::default(), target_state: RobotJointState::default() }));
        
        Self::controller(robot_lock.clone());

        robot_lock
    }

    fn controller(robot_lock: RobotLock){
        // Task to simulate robot movement
        tokio::spawn(async move {
            // Implement PD controller for this system.

            // Store previous positions to calculate velocity.

            let mut state_velocity: RobotJointState = RobotJointState::default();
            
            let mut count = 0;
            loop {
                count += 1;

                let start = Instant::now();
                //todo!("Add control logic. With max velocities.");

                let state;
                let target;
                {
                    let robot = robot_lock.read().await;
                    state = robot.state;
                    target = robot.target_state;
                }

                // Get the error
                let state_error = RobotJointState::clamped_sub(target, state);

                let mut state_acceleration = RobotJointState::default();

                // Calculate P
                state_acceleration.swing_rotation_deg = state_error.swing_rotation_deg*ANGLE_P;
                state_acceleration.lift_elevation_mm = state_error.lift_elevation_mm*LINEAR_P;
                state_acceleration.elbow_rotation_deg = state_error.elbow_rotation_deg*ANGLE_P;
                state_acceleration.wrist_rotation_deg = state_error.wrist_rotation_deg*ANGLE_P;
                state_acceleration.gripper_open_mm = state_error.gripper_open_mm*LINEAR_P;

                // if count % 50 == 0{
                //     println!("Vel {:?}", state_velocity);
                //     println!("A {:?}", state_acceleration);
                // }
                // Calculate D
                // if let Some(prev_state) = prev_state {
                    // Velocity is already know.
                    // let velocity = RobotJointState::clamped_sub(state, prev_state).val_div(CONTROLLER_LOOP_TIME_S);

                state_acceleration.swing_rotation_deg += -state_velocity.swing_rotation_deg*ANGLE_D;
                state_acceleration.lift_elevation_mm += -state_velocity.lift_elevation_mm*LINEAR_D;
                state_acceleration.elbow_rotation_deg += -state_velocity.elbow_rotation_deg*ANGLE_D;
                state_acceleration.wrist_rotation_deg += -state_velocity.wrist_rotation_deg*ANGLE_D;
                state_acceleration.gripper_open_mm += -state_velocity.gripper_open_mm*LINEAR_D;
                
                // if count % 50 == 0 {
                //     println!("V2 {:?}", state_velocity);
                //     println!("A2 {:?}", state_acceleration);
                // }
                // } 


                // Clamp acceleration.
                state_acceleration.swing_rotation_deg = state_acceleration.swing_rotation_deg.clamp(-MAX_ANGULAR_ACCELERATION, MAX_ANGULAR_ACCELERATION);
                state_acceleration.lift_elevation_mm = state_acceleration.lift_elevation_mm.clamp(-MAX_LINEAR_ACCELERATION, MAX_LINEAR_ACCELERATION);
                state_acceleration.elbow_rotation_deg = state_acceleration.elbow_rotation_deg.clamp(-MAX_ANGULAR_ACCELERATION, MAX_ANGULAR_ACCELERATION);
                state_acceleration.wrist_rotation_deg = state_acceleration.wrist_rotation_deg.clamp(-MAX_ANGULAR_ACCELERATION, MAX_ANGULAR_ACCELERATION);
                state_acceleration.gripper_open_mm = state_acceleration.gripper_open_mm.clamp(-MAX_LINEAR_ACCELERATION, MAX_LINEAR_ACCELERATION);

                // Apply acceleration
                state_velocity = state_velocity + state_acceleration.val_mul(CONTROLLER_LOOP_TIME_S);

                // Clamp velocity
                state_velocity.swing_rotation_deg = state_velocity.swing_rotation_deg.clamp(-MAX_ANGULAR_VELOCITY, MAX_ANGULAR_VELOCITY);
                state_velocity.lift_elevation_mm = state_velocity.lift_elevation_mm.clamp(-MAX_LINEAR_VELOCITY, MAX_LINEAR_VELOCITY);
                state_velocity.elbow_rotation_deg = state_velocity.elbow_rotation_deg.clamp(-MAX_ANGULAR_VELOCITY, MAX_ANGULAR_VELOCITY);
                state_velocity.wrist_rotation_deg = state_velocity.wrist_rotation_deg.clamp(-MAX_ANGULAR_VELOCITY, MAX_ANGULAR_VELOCITY);
                state_velocity.gripper_open_mm = state_velocity.gripper_open_mm.clamp(-MAX_LINEAR_VELOCITY, MAX_LINEAR_VELOCITY);

                // Update by applying velocity.
                {
                    let mut robot = robot_lock.write().await;
                    robot.set_state(state+state_velocity.val_mul(CONTROLLER_LOOP_TIME_S));
                }


                let loop_duration = Instant::now().duration_since(start);
                if let Some(sleep_duration) = Duration::from_millis(CONTROLLER_LOOP_TIME_MS).checked_sub(loop_duration) {
                    sleep(sleep_duration).await;
                };
            }
        });

    }

    // Rename
    fn set_state(&mut self, mut new_state: RobotJointState) {
        new_state.check_limits();
        
        self.state = new_state;
    }

    pub fn get_state(&self) -> RobotJointState{
        return self.state;
    }

    pub fn set_target_state(&mut self, mut target_state: RobotJointState) {
        target_state.check_limits();
        self.target_state = target_state;
    }

    fn ik(&self, coord_state: Coord4DOF) -> Option<RobotJointState> {
        println!("{:?}", coord_state);

        let mut target_state: RobotJointState = RobotJointState::default();

        target_state.lift_elevation_mm = coord_state.z*1000.0;

        // Get end effector position

        let end_effector_rad = degrees_to_radians(clamp_angle_diff(coord_state.theta));
        

        let end_effector_pos =  Coord2D{
            x: coord_state.x - GRIPPER_LENGTH_M*(end_effector_rad.cos()),
            y: coord_state.y - GRIPPER_LENGTH_M*(end_effector_rad.sin()),
        };

        let base_angle = (end_effector_pos.y).atan2(end_effector_pos.x);


        let c = (end_effector_pos.x.powf(2.0) + end_effector_pos.y.powf(2.0)).sqrt();
        if c > WRIST_LENGTH_M+ELBOW_LENGTH_M {return None;}

        
        let elbow_angle: f64 = -(PI - ((c.powf(2.0) - ELBOW_LENGTH_M.powf(2.0) - WRIST_LENGTH_M.powf(2.0))/(-2.0*ELBOW_LENGTH_M*WRIST_LENGTH_M)).acos());
        if elbow_angle.is_nan() {return None;}

        let swing_angle_local = ((WRIST_LENGTH_M.powf(2.0) - ELBOW_LENGTH_M.powf(2.0) - c.powf(2.0))/(-2.0*ELBOW_LENGTH_M*c)).acos();
        if swing_angle_local.is_nan() {return None;}
        

        let swing_angle = base_angle + swing_angle_local;
        

        target_state.swing_rotation_deg = radians_to_degrees(swing_angle);
        target_state.elbow_rotation_deg = radians_to_degrees(elbow_angle);
        target_state.wrist_rotation_deg = radians_to_degrees(end_effector_rad - elbow_angle - swing_angle);
        
        return Some(target_state);
    }

    pub fn set_coord_state(&mut self, coord_state: Coord4DOF) {
        if let Some(state) = self.ik(coord_state) {
            self.set_target_state(state);
        }
    }

    pub fn get_coord_state(&self) -> Coord4DOF {
        let state = self.state;
        
        let mut coords = Coord4DOF::default();
        coords.z = state.lift_elevation_mm/1000.0;

        let elbow_angle_rad = degrees_to_radians(state.swing_rotation_deg);
        let wrist_angle_rad = elbow_angle_rad+degrees_to_radians(state.elbow_rotation_deg);
        let gripper_angle_rad = wrist_angle_rad + degrees_to_radians(state.wrist_rotation_deg);

        // Calculate elbow coordinates
        coords.x += ELBOW_LENGTH_M * elbow_angle_rad.cos();
        coords.y += ELBOW_LENGTH_M * elbow_angle_rad.sin();

        // Calculate wrist coordinates relative to the elbow
        coords.x += WRIST_LENGTH_M * wrist_angle_rad.cos();
        coords.y += WRIST_LENGTH_M * wrist_angle_rad.sin();

        // Calculate gripper coordinates relative to the wrist
        coords.x += GRIPPER_LENGTH_M * gripper_angle_rad.cos();
        coords.y += GRIPPER_LENGTH_M * gripper_angle_rad.sin();

        return coords;
    }
}