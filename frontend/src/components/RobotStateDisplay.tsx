import { Coord3D, RobotState } from '../types/RobotTypes';

type RobotStateDisplayProps = {
  robotState: RobotState | null;
  coords: Coord3D | null
}

export const RobotStateDisplay = (props: RobotStateDisplayProps) => {

  return (
    <div>
      <h2>Current Robot State</h2>
      {(props.robotState && props.coords) ? (
        <div>
          <ul>
            <li>Swing Rotation (degrees): {props.robotState.swing_rotation_deg.toFixed(3)}</li>
            <li>Lift Elevation (mm): {props.robotState.lift_elevation_mm.toFixed(3)}</li>
            <li>Elbow Rotation (degrees): {props.robotState.elbow_rotation_deg.toFixed(3)}</li>
            <li>Wrist Rotation (degrees): {props.robotState.wrist_rotation_deg.toFixed(3)}</li>
            <li>Gripper Opening (mm): {props.robotState.gripper_open_mm.toFixed(3)}</li>
          </ul>
        
          <ul>
            <li>X (m): {props.coords.x}</li>
            <li>Y (m): {props.coords.y}</li>
            <li>Z (m): {props.coords.z}</li>
          </ul>
        </div>
      ) : (
        <p>No connection with server...</p>
      )}
    </div>
  );
};
