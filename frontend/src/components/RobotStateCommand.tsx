import React from 'react';
import socket from '../utils/socket';
import { useState } from 'react';
 
import { Coord4DOF as Coord4DOF, RobotState } from '../types/RobotTypes';

// Initial state values
const initialAccuatorState: RobotState = {
  swing_rotation_deg: 0,
  lift_elevation_mm: 0,
  elbow_rotation_deg: 0,
  wrist_rotation_deg: 0,
  gripper_open_mm: 0,
};

const initialCoordState: Coord4DOF = {
  x: 0,
  y: 0,
  z: 0,
  theta: 0,
};

export const RobotStateCommand = () => {
    // State to store the user inputs for the robot's actuators
  const [actuatorState, setActuatorState] = useState<RobotState>(initialAccuatorState);
  const [coordState, setCoordState] = useState<Coord4DOF>(initialCoordState);

  // Handle input change and validate that the value is a number
  const handleAccuatorChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;

    // Convert the value to a number
    const numericValue = Number(value);
    
    // Check if the input is a valid number
    if (!isNaN(numericValue)) {
      // Set the value to be the numeric output.
      e.target.value = String(numericValue);
      // Update the actuator state with the new value
      setActuatorState(prevState => ({
        ...prevState,
        [name]: numericValue,
      }));
    }
  };

  // Handle input change and validate that the value is a number
  const handleCoordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;

    // Convert the value to a number
    const numericValue = Number(value);

    // Check if the input is a valid number
    if (!isNaN(numericValue)) {
      // Set the value to be the numeric output.
      e.target.value = String(numericValue);
      // Update the actuator state with the new value
      setCoordState(prevState => ({
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

  // Function to handle sending the actuator state over the WebSocket
  const sendCoordState = () => {

    // Emit the "set actuator state" event with the current actuator state
    socket.emit('set coord state', coordState);
  };

  return (
    <div>
      <div>
        <h2>Control Robot Actuators</h2>
        <form>
          <label>
            Swing Rotation (degrees):
            <input
              type="number"
              step="any"
              name="swing_rotation_deg"
              value={actuatorState.swing_rotation_deg}
              onChange={handleAccuatorChange}
            />
          </label>
          <br />
          <label>
            Lift Elevation (mm):
            <input
              type="number"
              step="any"
              name="lift_elevation_mm"
              value={actuatorState.lift_elevation_mm}
              onChange={handleAccuatorChange}
            />
          </label>
          <br />
          <label>
            Elbow Rotation (degrees):
            <input
              type="number"
              step="any"
              name="elbow_rotation_deg"
              value={actuatorState.elbow_rotation_deg}
              onChange={handleAccuatorChange}
            />
          </label>
          <br />
          <label>
            Wrist Rotation (degrees):
            <input
              type="number"
              step="any"
              name="wrist_rotation_deg"
              value={actuatorState.wrist_rotation_deg}
              onChange={handleAccuatorChange}
            />
          </label>
          <br />
          <label>
            Gripper Opening (mm):
            <input
              type="number"
              step="any"
              name="gripper_open_mm"
              value={actuatorState.gripper_open_mm}
              onChange={handleAccuatorChange}
            />
          </label>
          <br />
          <button type="button" onClick={sendActuatorState}>
            Send Actuator State
          </button>
        </form>
      </div>

      <div>
        <h2>Control Robot Coords</h2>
        <form>
          <label>
            X (m):
            <input
              type="number"
              step="any"
              name="x"
              value={coordState.x}
              onChange={handleCoordChange}
            />
          </label>
          <br />
          <label>
            Y (m):
            <input
              type="number"
              step="any"
              name="y"
              value={coordState.y}
              onChange={handleCoordChange}
            />
          </label>
          <br />
          <label>
            Z (m):
            <input
              type="number"
              step="any"
              name="z"
              value={coordState.z}
              onChange={handleCoordChange}
            />
          </label>
          <label>
            Theta (deg):
            <input
              type="number"
              step="any"
              name="theta"
              value={coordState.theta}
              onChange={handleCoordChange}
            />
          </label>
          
          <button type="button" onClick={sendCoordState}>
            Send Coord State
          </button>
        </form>
      </div>
    </div>
    
  );
};