use std::ops::{Add, Mul, Sub};
use super::constants::*;

/// Converts a degree angle into a value in the range (-180, 180)
pub fn limit_angle(angle: f64) -> f64{
    return ((angle - 180.0 ).rem_euclid(360.0)).abs() -180.0 ;
}

pub fn clamp(val: f64, clamp_val: f64) -> f64{
    match val {
        x if x < -clamp_val => -clamp_val,
        x if x > clamp_val => clamp_val,
        _ => val,
    }
}

/// Returns the shortest difference between 2 angles.
pub fn shortest_angle_diff(angle1: f64, angle2: f64) -> f64 {
    let angle = angle1 - angle2;
    return match angle {
        angle if angle.abs() > 180.0 => {
            -(360.0 - angle.abs())*angle.signum()
        } 
        _ => {
            angle
        }
    }
}

/// Holds the state of each of the joints the make up the robot.
#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default)]
pub struct JointState {
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

impl JointState {
    /// Ensures robot's joint state is within the defined limits of what is possible.
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

    /// Multiple each joint value by `mul`.
    pub fn val_mul(&mut self, mul: f64) -> JointState{
        let mut output = JointState::default();
        
        output.swing_rotation_deg = self.swing_rotation_deg * mul;
        output.lift_elevation_mm = self.lift_elevation_mm * mul;
        output.elbow_rotation_deg = self.elbow_rotation_deg * mul;
        output.wrist_rotation_deg = self.wrist_rotation_deg * mul;
        output.gripper_open_mm = self.gripper_open_mm * mul;

        return output
    }

    pub fn val_div(&mut self, val: f64) -> JointState{
        let mut output = JointState::default();
        
        output.swing_rotation_deg = self.swing_rotation_deg / val;
        output.lift_elevation_mm = self.lift_elevation_mm / val;
        output.elbow_rotation_deg = self.elbow_rotation_deg / val;
        output.wrist_rotation_deg = self.wrist_rotation_deg / val;
        output.gripper_open_mm = self.gripper_open_mm / val;

        return output
    }

    /// Finds the difference between 2 `JointState`s.
    pub fn clamped_sub(lhs: JointState, rhs: JointState) -> JointState{
        
        let mut output = lhs - rhs;

        output.swing_rotation_deg = shortest_angle_diff(lhs.swing_rotation_deg, rhs.swing_rotation_deg);
        output.elbow_rotation_deg = shortest_angle_diff(lhs.elbow_rotation_deg, rhs.elbow_rotation_deg);
        output.wrist_rotation_deg = shortest_angle_diff(lhs.wrist_rotation_deg, rhs.wrist_rotation_deg);

        return output
    }
}

impl Add for JointState{
    type Output = JointState;

    fn add(self, rhs: JointState) -> JointState {
        JointState {
            swing_rotation_deg: self.swing_rotation_deg + rhs.swing_rotation_deg,
            lift_elevation_mm: self.lift_elevation_mm + rhs.lift_elevation_mm,
            elbow_rotation_deg: self.elbow_rotation_deg + rhs.elbow_rotation_deg,
            wrist_rotation_deg: self.wrist_rotation_deg + rhs.wrist_rotation_deg,
            gripper_open_mm: self.gripper_open_mm + rhs.gripper_open_mm,
        }
    }
}

impl Sub for JointState{
    type Output = JointState;

    fn sub(self, rhs: JointState) -> JointState {
        JointState {
            swing_rotation_deg: self.swing_rotation_deg - rhs.swing_rotation_deg,
            lift_elevation_mm: self.lift_elevation_mm - rhs.lift_elevation_mm,
            elbow_rotation_deg: self.elbow_rotation_deg - rhs.elbow_rotation_deg,
            wrist_rotation_deg: self.wrist_rotation_deg - rhs.wrist_rotation_deg,
            gripper_open_mm: self.gripper_open_mm - rhs.gripper_open_mm,
        }
    }
}

impl Mul for JointState {
    type Output = JointState;

    fn mul(self, rhs: JointState) -> JointState {
        JointState {
            swing_rotation_deg: self.swing_rotation_deg * rhs.swing_rotation_deg,
            lift_elevation_mm: self.lift_elevation_mm * rhs.lift_elevation_mm,
            elbow_rotation_deg: self.elbow_rotation_deg * rhs.elbow_rotation_deg,
            wrist_rotation_deg: self.wrist_rotation_deg * rhs.wrist_rotation_deg,
            gripper_open_mm: self.gripper_open_mm * rhs.gripper_open_mm,
        }
    }
}

/// A 4DOF corrdinate that contains an x, y, and z value as well as an angle.
#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default)]
pub struct Coord4DOF {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub theta: f64,
}

impl Coord4DOF {
    pub fn val_mul(&self, val: f64) -> Coord4DOF {
        let mut output = Coord4DOF::default();
        
        output.x = self.x * val;
        output.y = self.y * val;
        output.z = self.z * val;
        output.theta = self.theta * val;

        return output
    }

    /// Applies a linear and angular control value to ease in applying controller operations.
    pub fn apply_control(&self, linear: f64, angular: f64) -> Coord4DOF {
        let mut output = Coord4DOF::default();
        
        output.x = self.x * linear;
        output.y = self.y * linear;
        output.z = self.z * linear;
        output.theta = self.theta * angular;

        return output
    }

    /// Clamps the position and angle between the specified `clamp_pos` and `clamp_ang`.`
    pub fn clamp(&mut self, clamp_pos: f64, clamp_ang: f64) {
        self.x = clamp(self.x, clamp_pos);

        self.y = clamp(self.y, clamp_pos);
        self.z = clamp(self.z, clamp_pos);
        self.theta = clamp(self.theta, clamp_ang);
    }
}

impl Add for Coord4DOF {
    type Output = Coord4DOF;

    fn add(self, rhs: Coord4DOF) -> Coord4DOF {
        Coord4DOF {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            theta: self.theta + rhs.theta,
        }
    }
}

impl Sub for Coord4DOF {
    type Output = Coord4DOF;

    fn sub(self, rhs: Coord4DOF) -> Coord4DOF {
        Coord4DOF {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            theta: self.theta - rhs.theta,
        }
    }
}

// #[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default)]
// pub struct Coord2D {
//     pub x: f64,
//     pub y: f64,
// }

// impl Coord2D {
//     pub fn val_mul(&self, val: f64) -> Coord2D {
//         let mut output = Coord2D::default();
        
//         output.x = self.x * val;
//         output.y = self.y * val;

//         return output
//     }

//     pub fn clamp(&mut self, clamp_val: f64) {
//         self.x = clamp(self.x, clamp_val);
//         self.y = clamp(self.y, clamp_val);
//     }
// }

// impl Add for Coord2D {
//     type Output = Coord2D;

//     fn add(self, rhs: Coord2D) -> Coord2D {
//         Coord2D {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

// impl Sub for Coord2D {
//     type Output = Coord2D;

//     fn sub(self, rhs: Coord2D) -> Coord2D {
//         Coord2D {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//         }
//     }
// }

/// Holds both the joint and base state of the robot.
#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, Default)]
pub struct RobotState {
    pub joint_state: JointState,
    pub base_state: Coord4DOF
}
