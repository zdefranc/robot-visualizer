import React from 'react';
import socket from '../utils/socket';
import { useState } from 'react';
 
import { RobotState } from '../types/RobotTypes';

// Initial state values
const initialState: RobotState = {
    swing_rotation_deg: 0,
    lift_elevation_mm: 0,
    elbow_rotation_deg: 0,
    wrist_rotation_deg: 0,
    gripper_open_mm: 0,
  };

export const RobotStateCommand = () => {
    // State to store the user inputs for the robot's actuators
  const [actuatorState, setActuatorState] = useState<RobotState>(initialState);
  const [error, setError] = useState<string | null>(null);

  // Handle input change and validate that the value is a number
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;

    // Convert the value to a number
    const numericValue = Number(value);

    // Check if the input is a valid number
    if (isNaN(numericValue)) {
      setError(`Invalid value for ${name}. Please enter a number.`);
    } else {
      setError(null);
      // Update the actuator state with the new value
      setActuatorState(prevState => ({
        ...prevState,
        [name]: numericValue,
      }));
    }
  };

  // Function to handle sending the actuator state over the WebSocket
  const sendActuatorState = () => {

    // Emit the "set actuator state" event with the current actuator state
    socket.emit('set actuator state', actuatorState);
  };

  return (
    <div>
      <h2>Control Robot Actuators</h2>
      <form>
        <label>
          Swing Rotation (degrees):
          <input
            type="text"
            name="swing_rotation_deg"
            value={actuatorState.swing_rotation_deg}
            onChange={handleChange}
          />
        </label>
        <br />
        <label>
          Lift Elevation (mm):
          <input
            type="text"
            name="lift_elevation_mm"
            value={actuatorState.lift_elevation_mm}
            onChange={handleChange}
          />
        </label>
        <br />
        <label>
          Elbow Rotation (degrees):
          <input
            type="text"
            name="elbow_rotation_deg"
            value={actuatorState.elbow_rotation_deg}
            onChange={handleChange}
          />
        </label>
        <br />
        <label>
          Wrist Rotation (degrees):
          <input
            type="text"
            name="wrist_rotation_deg"
            value={actuatorState.wrist_rotation_deg}
            onChange={handleChange}
          />
        </label>
        <br />
        <label>
          Gripper Opening (mm):
          <input
            type="text"
            name="gripper_open_mm"
            value={actuatorState.gripper_open_mm}
            onChange={handleChange}
          />
        </label>
        <br />
        <button type="button" onClick={sendActuatorState}>
          Send Actuator State
        </button>
      </form>
      {error && <p style={{ color: 'red' }}>{error}</p>}
    </div>
  );
};