import { useState, useEffect } from 'react';
import socket from '../utils/socket';

import { RobotStateDisplay } from './RobotStateDisplay';
import { RobotStateCommand } from './RobotStateCommand';
import RobotVisualization from './RobotVisualization';

import { RobotState } from '../types/RobotTypes';

export const Robot = () => {
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
        <div>
            <RobotStateDisplay robotState={robotState} />
            <RobotStateCommand />
        </div>
        <div>
            <RobotVisualization robotState={robotState} />
        </div>
    </div>
  );
};
