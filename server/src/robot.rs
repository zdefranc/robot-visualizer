use std::ops::{Add, Sub};
use tokio::sync::RwLock;
use tracing::info;
use std::sync::Arc;
use tokio::time::{sleep, Instant, Duration};

/// Max Angular velcoity (deg/sec)
const MAX_ANGULAR_VELOCITY: f64 = 0.2;
/// Max linear velocity (mm/sec)
const MAX_LINEAR_VELOCITY: f64 = 5.0;

/// Max Angular velcoity (deg/sec)
const MAX_ANGULAR_ACCELERATION: f64 = 0.2;
/// Max linear velocity (mm/sec)
const MAX_LINEAR_ACCELERATION: f64 = 5.0;

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default)]
pub struct RobotState {
    /// Swing rotation (degrees).
    pub swing_rotation_deg: f64,
    /// Lift elevation (mm).
    pub lift_elevation_mm: f64,
    /// Elbow rotation (degrees).
    pub elbow_rotation_deg: f64,
    /// Wrist rotation (degrees).
    pub wrist_rotation_deg: f64,
    /// Gripper opening (mm).
    pub gripper_open_mm: f64,
}

impl Add for RobotState{
    type Output = RobotState;

    fn add(self, rhs: RobotState) -> RobotState {
        RobotState {
            swing_rotation_deg: self.swing_rotation_deg + rhs.swing_rotation_deg,
            lift_elevation_mm: self.lift_elevation_mm + rhs.lift_elevation_mm,
            elbow_rotation_deg: self.elbow_rotation_deg + rhs.elbow_rotation_deg,
            wrist_rotation_deg: self.wrist_rotation_deg + rhs.wrist_rotation_deg,
            gripper_open_mm: self.gripper_open_mm + rhs.gripper_open_mm,
        }
    }
}

impl Sub for RobotState{
    type Output = RobotState;

    fn sub(self, rhs: RobotState) -> RobotState {
        RobotState {
            swing_rotation_deg: self.swing_rotation_deg - rhs.swing_rotation_deg,
            lift_elevation_mm: self.lift_elevation_mm - rhs.lift_elevation_mm,
            elbow_rotation_deg: self.elbow_rotation_deg - rhs.elbow_rotation_deg,
            wrist_rotation_deg: self.wrist_rotation_deg - rhs.wrist_rotation_deg,
            gripper_open_mm: self.gripper_open_mm - rhs.gripper_open_mm,
        }
    }
}

pub type RobotLock = Arc<RwLock<Robot>>;

pub struct Robot {
    pub state: RobotState,
    pub target_state: RobotState,
}

impl Robot {
    pub fn new() -> Arc<RwLock<Self>> {
        let robot_lock: RobotLock = Arc::new(RwLock::new(Self { state: RobotState::default(), target_state: RobotState::default() }));
        
        Self::controller(robot_lock.clone());

        robot_lock
    }

    fn controller(robot_lock: RobotLock){
        // Task to simulate robot movement
        tokio::spawn(async move {
            loop {
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
                let state_error = target - state;

                {
                    let mut robot = robot_lock.write().await;
                    robot.set_state(state_error+state);
                }

                // Maybe use a min acceleration value


                let loop_duration = Instant::now().duration_since(start);
                if let Some(sleep_duration) = Duration::from_millis(100).checked_sub(loop_duration) {
                    sleep(sleep_duration).await;
                };
            }
        });

    }

    // Rename
    fn set_state(&mut self, new_state: RobotState) {
        self.state = new_state;
    }

    pub fn set_target_state(&mut self, target_state: RobotState){
        self.target_state = target_state;
        println!("Test {}", self.target_state.elbow_rotation_deg)
    }
}