
import { useState } from 'react';
import styles from '../css/RobotStateCommand.module.css'

type StateInputProps = {
    handleChange: React.ChangeEventHandler<HTMLInputElement>;
    name: string,
    label_name: string
}


export const StateInput = (props: StateInputProps) => {
    const [jointState, setJointState] = useState<string>("0");
    
    // Handle input change for the joint state and validate that the value is a number
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;

        setJointState(value);
    
        props.handleChange(e);
      };

    return (
        <label className={styles['state-label']}>
            <text>{props.label_name}</text>
            <input 
                className={styles['state-input']}
                type='number'
                step="any"
                name={props.name}
                value={jointState}
                onChange={handleChange}
            />
          </label>
    );
  };