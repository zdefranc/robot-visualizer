use std::ops::{Add, Mul, Sub};
use super::constants::*;

fn limit_angle(angle: f64) -> f64{
    return ((angle - 180.0 ).rem_euclid(360.0)).abs() -180.0 ;
}

// Bad name!
fn clamp_angle_diff(angle: f64) -> f64 {
    return match (angle) {
        angle if angle.abs() > 180.0 => {
            -(360.0 - angle.abs())*angle.signum()
        } 
        _ => {
            angle
        }
    }
}

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

impl RobotJointState {
    pub fn check_limits(&mut self) {
        self.swing_rotation_deg = limit_angle(self.swing_rotation_deg);
        self.elbow_rotation_deg = limit_angle(self.elbow_rotation_deg);
        self.wrist_rotation_deg = limit_angle(self.wrist_rotation_deg);

        self.lift_elevation_mm = match self.lift_elevation_mm {
            val if LIFT_HEIGHT_MM < val => LIFT_HEIGHT_MM,
            val if 0.0 > val => 0.0,
            _ => self.lift_elevation_mm
        };

        self.gripper_open_mm = match self.gripper_open_mm {
            val if GRIPPER_WIDTH_MM < val => GRIPPER_WIDTH_MM,
            val if 0.0 > val => 0.0,
            _ => self.gripper_open_mm
        };

    }

    pub fn val_mul(&mut self, val: f64) -> RobotJointState{
        let mut output = RobotJointState::default();
        
        output.swing_rotation_deg = self.swing_rotation_deg * val;
        output.lift_elevation_mm = self.lift_elevation_mm * val;
        output.elbow_rotation_deg = self.elbow_rotation_deg * val;
        output.wrist_rotation_deg = self.wrist_rotation_deg * val;
        output.gripper_open_mm = self.gripper_open_mm * val;

        return output
    }

    pub fn val_div(&mut self, val: f64) -> RobotJointState{
        let mut output = RobotJointState::default();
        
        output.swing_rotation_deg = self.swing_rotation_deg / val;
        output.lift_elevation_mm = self.lift_elevation_mm / val;
        output.elbow_rotation_deg = self.elbow_rotation_deg / val;
        output.wrist_rotation_deg = self.wrist_rotation_deg / val;
        output.gripper_open_mm = self.gripper_open_mm / val;

        return output
    }

    pub fn clamped_sub(lhs: RobotJointState, rhs: RobotJointState) -> RobotJointState{
        
        let mut output = lhs - rhs;

        output.swing_rotation_deg = clamp_angle_diff(output.swing_rotation_deg);
        output.elbow_rotation_deg = clamp_angle_diff(output.elbow_rotation_deg);
        output.wrist_rotation_deg = clamp_angle_diff(output.wrist_rotation_deg);

        return output
    }
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

impl Mul for RobotJointState {
    type Output = RobotJointState;

    fn mul(self, rhs: RobotJointState) -> RobotJointState {
        RobotJointState {
            swing_rotation_deg: self.swing_rotation_deg * rhs.swing_rotation_deg,
            lift_elevation_mm: self.lift_elevation_mm * rhs.lift_elevation_mm,
            elbow_rotation_deg: self.elbow_rotation_deg * rhs.elbow_rotation_deg,
            wrist_rotation_deg: self.wrist_rotation_deg * rhs.wrist_rotation_deg,
            gripper_open_mm: self.gripper_open_mm * rhs.gripper_open_mm,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default)]
pub struct Coord3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}