export type RobotState = {
    swing_rotation_deg: number;
    lift_elevation_mm: number;
    elbow_rotation_deg: number;
    wrist_rotation_deg: number;
    gripper_open_mm: number;
  };

export type Coord3D = {
    x : number;
    y : number;
    z : number;
};