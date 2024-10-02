import React from 'react';
import socket from '../utils/socket';
import { useState } from 'react';
 
import { Coord2D, Coord4DOF as Coord4DOF, JointState } from '../types/RobotTypes';

// Initial state values
const initialAccuatorState: JointState = {
  swing_rotation_deg: 0,
  lift_elevation_mm: 0,
  elbow_rotation_deg: 0,
  wrist_rotation_deg: 0,
  gripper_open_mm: 0,
};

const initialCoord4DOFState: Coord4DOF = {
  x: 0,
  y: 0,
  z: 0,
  theta: 0,
};

const initialCoord2DState: Coord4DOF = {
  x: 0,
  y: 0,
  z: 0,
  theta: 0,
};

export const RobotStateCommand = () => {
    // State to store the user inputs for the robot's actuators
  const [actuatorState, setActuatorState] = useState<JointState>(initialAccuatorState);
  const [coordState, setCoordState] = useState<Coord4DOF>(initialCoord4DOFState);
  const [baseCoord, setBaseCoord] = useState<Coord4DOF>(initialCoord2DState);

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
  const handleCoord4DOFChange = (e: React.ChangeEvent<HTMLInputElement>) => {
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

  // Handle input change and validate that the value is a number
  const handleBaseCoordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;

    // Convert the value to a number
    const numericValue = Number(value);

    // Check if the input is a valid number
    if (!isNaN(numericValue)) {
      // Set the value to be the numeric output.
      e.target.value = String(numericValue);
      // Update the actuator state with the new value
      setBaseCoord(prevState => ({
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

    // Function to handle sending the actuator state over the WebSocket
  const sendBaseCoordState = () => {

    // Emit the "set actuator state" event with the current actuator state
    socket.emit('set base state', baseCoord);
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
              onChange={handleCoord4DOFChange}
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
              onChange={handleCoord4DOFChange}
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
              onChange={handleCoord4DOFChange}
            />
          </label>
          <label>
            Theta (deg):
            <input
              type="number"
              step="any"
              name="theta"
              value={coordState.theta}
              onChange={handleCoord4DOFChange}
            />
          </label>
          
          <button type="button" onClick={sendCoordState}>
            Send Coord State
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
              value={baseCoord.x}
              onChange={handleBaseCoordChange}
            />
          </label>
          <br />
          <label>
            Y (m):
            <input
              type="number"
              step="any"
              name="y"
              value={baseCoord.y}
              onChange={handleBaseCoordChange}
            />
          </label>
          <label>
            Z (m):
            <input
              type="number"
              step="any"
              name="z"
              value={baseCoord.z}
              onChange={handleBaseCoordChange}
            />
          </label>
          <label>
            Theta (deg):
            <input
              type="number"
              step="any"
              name="theta"
              value={baseCoord.theta}
              onChange={handleBaseCoordChange}
            />
          </label>
          
          <button type="button" onClick={sendBaseCoordState}>
            Send Coord State
          </button>
          
        </form>
      </div>
    </div>
    
  );
};