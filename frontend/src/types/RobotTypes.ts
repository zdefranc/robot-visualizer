export type JointState = {
    swing_rotation_deg: number;
    lift_elevation_mm: number;
    elbow_rotation_deg: number;
    wrist_rotation_deg: number;
    gripper_open_mm: number;
  };

export type Coord4DOF = {
    x : number;
    y : number;
    z : number;
    theta: number;
};

export type Coord2D = {
    x : number;
    y : number;
};

export type RobotState = {
  joint_state: JointState;
  base_state: Coord4DOF;
};
