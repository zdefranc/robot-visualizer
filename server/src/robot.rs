use tokio::sync::RwLock;
use tracing::info;
use std::sync::Arc;
use tokio::time::{sleep, Instant, Duration};

#[derive(serde::Serialize, Clone, Debug, Default)]
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


pub struct Robot {
    pub state: RobotState,
    pub target_state: RobotState,
}

impl Robot {
    pub fn new() -> Arc<RwLock<Self>> {
        let robot_lock: Arc<RwLock<Robot>> = Arc::new(RwLock::new(Self { state: RobotState::default(), target_state: RobotState::default() }));
        
        Self::controller(robot_lock.clone());

        robot_lock
    }

    fn controller(robot_lock: Arc<RwLock<Robot>>){
        // Task to simulate robot movement
        tokio::spawn(async move {
            loop {
                let start = Instant::now();
                info!("Run");
                //todo!("Add control logic. With max velocities.");
                {
                    let robot = robot_lock.write().await.state.elbow_rotation_deg += 0.01;
                }

                let loop_duration = Instant::now().duration_since(start);
                if let Some(sleep_duration) = Duration::from_millis(100).checked_sub(loop_duration) {
                    sleep(sleep_duration).await;
                }
            }
        });

    }
}