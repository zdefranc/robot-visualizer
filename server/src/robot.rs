use tokio::sync::RwLock;
use std::sync::Arc;

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

#[derive(Default)]
pub struct Robot {
    pub state: RobotState
}