import { RobotState } from '../types/RobotTypes';

type RobotStateDisplayProps = {
  robotState: RobotState | null;
}

export const RobotStateDisplay = (props: RobotStateDisplayProps) => {

  return (
    <div>
      <h2>Current Robot State</h2>
      {props.robotState ? (
        <ul>
          <li>Swing Rotation (degrees): {props.robotState.swing_rotation_deg}</li>
          <li>Lift Elevation (mm): {props.robotState.lift_elevation_mm}</li>
          <li>Elbow Rotation (degrees): {props.robotState.elbow_rotation_deg}</li>
          <li>Wrist Rotation (degrees): {props.robotState.wrist_rotation_deg}</li>
          <li>Gripper Opening (mm): {props.robotState.gripper_open_mm}</li>
        </ul>
      ) : (
        <p>No connection with server...</p>
      )}
    </div>
  );
};
