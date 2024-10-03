import React from 'react';
import socket from '../utils/socket';
import { useState } from 'react';
 
import { Coord4DOF as Coord4DOF, JointState } from '../types/RobotTypes';
import styles from '../css/RobotStateCommand.module.css'
import { StateInput } from './StateInput';

// Initial state values
const initialJointState: JointState = {
  swing_rotation_deg: 0,
  lift_elevation_mm: 0,
  elbow_rotation_deg: 0,
  wrist_rotation_deg: 0,
  gripper_open_mm: 0,
};

// Initial state values
const initialCoord4DOFState: Coord4DOF = {
  x: 0,
  y: 0,
  z: 0,
  theta: 0,
};

export const RobotStateCommand = () => {
  // Store the state of the values to request.
  const [jointState, setJointState] = useState<JointState>(initialJointState);
  const [endEffectorState, setEndEffectorState] = useState<Coord4DOF>(initialCoord4DOFState);
  const [baseCoord, setBaseCoord] = useState<Coord4DOF>(initialCoord4DOFState);

  // Handle input change for the joint state and validate that the value is a number
  const handleJointChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;

    // Convert the value to a number
    const numericValue = Number(value);
    
    // Check if the input is a valid number
    if (!isNaN(numericValue)) {
      // Update the joint state with the new value
      setJointState(prevState => ({
        ...prevState,
        [name]: numericValue,
      }));
    }
  };

  // Handle input change for the end effector and validate that the value is a number
  const handleEndEffectorChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;

    // Convert the value to a number
    const numericValue = Number(value);

    // Check if the input is a valid number
    if (!isNaN(numericValue)) {
      // Update the end effector state with the new value
      setEndEffectorState(prevState => ({
        ...prevState,
        [name]: numericValue,
      }));
    }
  };

  // Handle input change for the base and validate that the value is a number
  const handleBaseCoordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;

    // Convert the value to a number
    const numericValue = Number(value);

    // Check if the input is a valid number
    if (!isNaN(numericValue)) {
      // Update the base state with the new value
      setBaseCoord(prevState => ({
        ...prevState,
        [name]: numericValue,
      }));
    }
  };

  // Function to handle sending the joint state over the WebSocket
  const sendJointState = () => {
    socket.emit('set joint state', jointState);
  };

  // Function to handle sending the end effector state over the WebSocket
  const sendCoordState = () => {
    socket.emit('set coord state', endEffectorState);
  };

    // Function to handle sending the base state over the WebSocket
  const sendBaseCoordState = () => {
    socket.emit('set base state', baseCoord);
  };

  return (
    <div className={styles['command-conatiner']}>
      <div>
        <h2 className={styles['title']}>Control Robot Actuators</h2>
        <form>
          <StateInput label_name="Swing Rotation (degrees):" handleChange={handleJointChange} name='swing_rotation_deg'/>
          <br className={styles['break']}/>
          <StateInput label_name="Lift Elevation (mm):" handleChange={handleJointChange} name='lift_elevation_mm'/>
          
          <br className={styles['break']}/>
          <StateInput label_name="Elbow Rotation (degrees):" handleChange={handleJointChange} name='elbow_rotation_deg'/>
          
          <br className={styles['break']}/>
          <StateInput label_name="Wrist Rotation (degrees):" handleChange={handleJointChange} name='wrist_rotation_deg'/>
          
          <br className={styles['break']}/>
          <StateInput label_name="Gripper Opening (mm):" handleChange={handleJointChange} name='gripper_open_mm'/>
          
          <br className={styles['break']}/>
          <button className={styles['button']} type="button" onClick={sendJointState}>
            Send Joint State
          </button>
        </form>
      </div>

      <div>
        <h2 className={styles['title']}>Control Robot End Effector</h2>
        <form>
          <StateInput label_name="X (m):" handleChange={handleEndEffectorChange} name='x'/>
          
          <br className={styles['break']}/>
          <StateInput label_name="Y (m):" handleChange={handleEndEffectorChange} name='y'/>
          
          <br className={styles['break']}/>
          <StateInput label_name="Z (m):" handleChange={handleEndEffectorChange} name='z'/>
          
          <br className={styles['break']}/>
          <StateInput label_name="Theta (deg):" handleChange={handleEndEffectorChange} name='theta'/>
          
          <br className={styles['break']}/>
          <button className={styles['button']} type="button" onClick={sendCoordState}>
            Send Coord State
          </button>
          
        </form>
      </div>
      
      <div>
        <h2 className={styles['title']}>Control Robot Base</h2>
        <form>
          <StateInput label_name="X (m):" handleChange={handleBaseCoordChange} name='x'/>
          <br className={styles['break']}/>
          <StateInput label_name="Y (m):" handleChange={handleBaseCoordChange} name='y'/>
          <br className={styles['break']}/>
          <StateInput label_name="Z (m):" handleChange={handleBaseCoordChange} name='z'/>
          <br className={styles['break']}/>
          <StateInput label_name="Theta (deg):" handleChange={handleBaseCoordChange} name='theta'/>
          
          <br className={styles['break']}/>
          <button className={styles['button']} type="button" onClick={sendBaseCoordState}>
            Send Coord State
          </button>
          
        </form>
      </div>
    </div>
    
  );
};