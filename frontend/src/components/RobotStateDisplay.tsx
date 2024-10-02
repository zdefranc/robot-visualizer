import { Coord4DOF, JointState, RobotState } from '../types/RobotTypes';

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
            <li>Swing Rotation (degrees): {props.robotState.joint_state.swing_rotation_deg.toFixed(2)}</li>
            <li>Lift Elevation (mm): {props.robotState.joint_state.lift_elevation_mm.toFixed(2)}</li>
            <li>Elbow Rotation (degrees): {props.robotState.joint_state.elbow_rotation_deg.toFixed(2)}</li>
            <li>Wrist Rotation (degrees): {props.robotState.joint_state.wrist_rotation_deg.toFixed(2)}</li>
            <li>Gripper Opening (mm): {props.robotState.joint_state.gripper_open_mm.toFixed(2)}</li>
            <li>Base x (mm): {props.robotState.base_state.x.toFixed(2)}</li>
            <li>Base y (mm): {props.robotState.base_state.y.toFixed(2)}</li>
            <li>Base z (mm): {props.robotState.base_state.z.toFixed(2)}</li>
            <li>Base theta (mm): {props.robotState.base_state.theta.toFixed(2)}</li>
          </ul>
        
          <ul>
            <li>X (m): {props.coords.x.toFixed(3)}</li>
            <li>Y (m): {props.coords.y.toFixed(3)}</li>
            <li>Z (m): {props.coords.z.toFixed(3)}</li>
            <li>Theta (deg): {props.coords.theta.toFixed(3)}</li>
          </ul>
        </div>
      ) : (
        <p>No connection with server...</p>
      )}
    </div>
  );
};
