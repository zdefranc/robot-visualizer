import { Coord4DOF, RobotState } from '../types/RobotTypes';

type RobotStateDisplayProps = {
  robotState: RobotState | null;
  coords: Coord4DOF | null
}

export const RobotStateDisplay = (props: RobotStateDisplayProps) => {

  return (
    <div>
      <h2>Current Robot State</h2>
      {(props.robotState && props.coords) ? (
        <div>
          <ul>
            <li>Swing Rotation (degrees): {props.robotState.swing_rotation_deg.toFixed(2)}</li>
            <li>Lift Elevation (mm): {props.robotState.lift_elevation_mm.toFixed(2)}</li>
            <li>Elbow Rotation (degrees): {props.robotState.elbow_rotation_deg.toFixed(2)}</li>
            <li>Wrist Rotation (degrees): {props.robotState.wrist_rotation_deg.toFixed(2)}</li>
            <li>Gripper Opening (mm): {props.robotState.gripper_open_mm.toFixed(2)}</li>
          </ul>
        
          <ul>
            <li>X (m): {props.coords.x.toFixed(3)}</li>
            <li>Y (m): {props.coords.y.toFixed(3)}</li>
            <li>Z (m): {props.coords.z.toFixed(3)}</li>
          </ul>
        </div>
      ) : (
        <p>No connection with server...</p>
      )}
    </div>
  );
};
