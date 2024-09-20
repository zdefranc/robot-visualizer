use std::ops::{Add, Sub};

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default)]
pub struct RobotJointState {
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

impl Add for RobotJointState{
    type Output = RobotJointState;

    fn add(self, rhs: RobotJointState) -> RobotJointState {
        RobotJointState {
            swing_rotation_deg: self.swing_rotation_deg + rhs.swing_rotation_deg,
            lift_elevation_mm: self.lift_elevation_mm + rhs.lift_elevation_mm,
            elbow_rotation_deg: self.elbow_rotation_deg + rhs.elbow_rotation_deg,
            wrist_rotation_deg: self.wrist_rotation_deg + rhs.wrist_rotation_deg,
            gripper_open_mm: self.gripper_open_mm + rhs.gripper_open_mm,
        }
    }
}

impl Sub for RobotJointState{
    type Output = RobotJointState;

    fn sub(self, rhs: RobotJointState) -> RobotJointState {
        RobotJointState {
            swing_rotation_deg: self.swing_rotation_deg - rhs.swing_rotation_deg,
            lift_elevation_mm: self.lift_elevation_mm - rhs.lift_elevation_mm,
            elbow_rotation_deg: self.elbow_rotation_deg - rhs.elbow_rotation_deg,
            wrist_rotation_deg: self.wrist_rotation_deg - rhs.wrist_rotation_deg,
            gripper_open_mm: self.gripper_open_mm - rhs.gripper_open_mm,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default)]
pub struct Robot3DState {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}