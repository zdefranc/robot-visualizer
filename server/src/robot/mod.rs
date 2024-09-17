pub mod robot_state;

use robot_state::RobotState;
use tokio::sync::RwLock;
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