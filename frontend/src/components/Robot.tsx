import { useState, useEffect } from 'react';
import socket from '../utils/socket';

import { RobotStateDisplay } from './RobotStateDisplay';
import { RobotStateCommand } from './RobotStateCommand';
import RobotVisualization from './RobotVisualization';

import { Coord4DOF, RobotState } from '../types/RobotTypes';

import styles from '../css/Robot.module.css'

export const Robot = () => {
  const [robotState, setRobotState] = useState<RobotState | null>(null);
  const [coords, setCoords] = useState<Coord4DOF | null>(null);

  useEffect(() => {
    socket.on('joint state', (data: RobotState) => {
      setRobotState(data);  // Update the joints with the received data.
    });

    socket.on('base coords', (data: Coord4DOF) => {
      setCoords(data);  // Update the base with the received data.
    });

    socket.on('disconnect', () => {
      setRobotState(null); 
    })

    // Cleanup on component unmount
    return () => {
      socket.off('joint state'); 
      socket.off('base coords');
    };
  }, []);

  return (
    <div className={styles['robot-container']}>
        <div className={styles['robot-div']}>
            <RobotVisualization robotState={robotState} />
        </div>
        <div className={styles['command-div']}>
            <RobotStateDisplay robotState={robotState} coords={coords}/>
            <RobotStateCommand />
        </div>
    </div>
  );
};
