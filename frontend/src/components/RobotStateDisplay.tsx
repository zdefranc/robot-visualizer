import { useState, useEffect } from 'react';
import socket from '../utils/socket';

// This is a duplicate !!
type RobotState = {
  swing_rotation_deg: number;
  lift_elevation_mm: number;
  elbow_rotation_deg: number;
  wrist_rotation_deg: number;
  gripper_open_mm: number;
};

export const RobotStateDisplay = () => {
  // State to store the robot's joint state received from the server
  const [robotState, setRobotState] = useState<RobotState | null>(null);

  useEffect(() => {
    // Listen for the "state" message from the server
    socket.on('state', (data: RobotState) => {
      setRobotState(data);  // Update the state with the received data
    });

    socket.on('disconnect', () => {
      setRobotState(null);
    })

    // Cleanup on component unmount
    return () => {
      socket.off('state');  // Remove the event listener when component is unmounted
    };
  }, []);

  return (
    <div>
      <h2>Current Robot State</h2>
      {robotState ? (
        <ul>
          <li>Swing Rotation (degrees): {robotState.swing_rotation_deg}</li>
          <li>Lift Elevation (mm): {robotState.lift_elevation_mm}</li>
          <li>Elbow Rotation (degrees): {robotState.elbow_rotation_deg}</li>
          <li>Wrist Rotation (degrees): {robotState.wrist_rotation_deg}</li>
          <li>Gripper Opening (mm): {robotState.gripper_open_mm}</li>
        </ul>
      ) : (
        <p>No connection with server...</p>
      )}
    </div>
  );
};
