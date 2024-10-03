import { Coord4DOF, RobotState } from '../types/RobotTypes';

import styles from '../css/RobotStateDisplay.module.css'

type RobotStateDisplayProps = {
  robotState: RobotState | null;
  coords: Coord4DOF | null
}

export const RobotStateDisplay = (props: RobotStateDisplayProps) => {


  return (
    <div className={styles['state-parent-div']}>
      <h2 className={styles['title']}>Robot State</h2>
      {(props.robotState && props.coords) ? (
        <div className={styles['state-div']}>
          <ul className={styles['joint-list']}>
            <li className={styles['state-header']}><strong>Joints</strong></li>
            <li className={styles['state-item']}>Swing Rotation (degrees): {props.robotState.joint_state.swing_rotation_deg.toFixed(1)}</li>
            <li className={styles['state-item']}>Lift Elevation (mm): {props.robotState.joint_state.lift_elevation_mm.toFixed(0)}</li>
            <li className={styles['state-item']}>Elbow Rotation (degrees): {props.robotState.joint_state.elbow_rotation_deg.toFixed(1)}</li>
            <li className={styles['state-item']}>Wrist Rotation (degrees): {props.robotState.joint_state.wrist_rotation_deg.toFixed(1)}</li>
            <li className={styles['state-item']}>Gripper Opening (mm): {props.robotState.joint_state.gripper_open_mm.toFixed(0)}</li>
          </ul>
          <ul className={styles['state-list']}>
            <li className={styles['state-header']}><strong>End Effector</strong></li>
            <li className={styles['state-item']}>X (m): {props.coords.x.toFixed(3)}</li>
            <li className={styles['state-item']}>Y (m): {props.coords.y.toFixed(3)}</li>
            <li className={styles['state-item']}>Z (m): {props.coords.z.toFixed(3)}</li>
            <li className={styles['state-item']}>Theta (deg): {props.coords.theta.toFixed(1)}</li>
          </ul>
          <ul className={styles['state-list']}>
            <li className={styles['state-header']}><strong>Base</strong></li>
            <li className={styles['state-item']}>X (m): {props.robotState.base_state.x.toFixed(3)}</li>
            <li className={styles['state-item']}>Y (m): {props.robotState.base_state.y.toFixed(3)}</li>
            <li className={styles['state-item']}>Z (m): {props.robotState.base_state.z.toFixed(3)}</li>
            <li className={styles['state-item']}>Theta (deg): {props.robotState.base_state.theta.toFixed(1)}</li>
          </ul>
        </div>
      ) : (
        <p>No connection with server...</p>
      )}
    </div>
  );
};
