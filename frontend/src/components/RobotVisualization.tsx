import * as THREE from 'three';
import { RobotState } from '../types/RobotTypes';

import { useEffect, useRef } from "react";

const BASE_HEIGHT = 0.3;

const LIFT_HEIGHT = 4;

const ELBOW_LENGTH = 2;

const WRIST_LENGTH = 1;

const GRIPPER_BASE_LENGTH = 0.4;

const GRIPPER_BASE_WIDTH = 0.4;

const GRIPPERS_WIDTH = 0.05;

const GRIPPERS_LENGTH = 0.2;

type RobotVisualizationProps = {
  robotState: RobotState | null;
}

function RobotVisualization(props: RobotVisualizationProps) {
  const refContainer = useRef<HTMLDivElement>(null);
  const robotStateRef = useRef<RobotState | null>(props.robotState); // Store the latest robot state

  // Update robotStateRef on every prop change
  // Is this right?
  useEffect(() => {
    robotStateRef.current = props.robotState;
  }, [props.robotState]);

  useEffect(() => {
    console.log("Mount");

    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0xeeeeee);

    // Create a camera
    const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    camera.position.z = 10;
    camera.position.y = 2;
    camera.position.x = 3;

    const gridHelper = new THREE.GridHelper(100, 100);
    gridHelper.position.y = 0;
    scene.add(gridHelper);
    
    // Create the renderer
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(window.innerWidth/2, window.innerHeight);
    document.body.appendChild(renderer.domElement);

    // use ref as a mount point of the Three.js scene instead of the document.body
    // This keeps multiple simulations from appearing.
    refContainer.current && refContainer.current.appendChild( renderer.domElement );
    
    // Add light to the scene
    const light = new THREE.DirectionalLight(0xffffff, 1);
    light.position.set(5, 10, 7.5);
    scene.add(light);

    // Create the robot arm parts (cylinders to represent each segment)
    const material = new THREE.MeshStandardMaterial({ color: 0x0077ff });

    // Base
    const base = new THREE.Mesh(new THREE.CylinderGeometry(0.8, 0.8, BASE_HEIGHT, 32), material);
    base.position.set(0, BASE_HEIGHT/2, 0);
    scene.add(base);
    
    // swing joint
    const swingJoint = new THREE.Object3D();
    swingJoint.position.set(0, BASE_HEIGHT/2, 0)
    base.add(swingJoint);

    // Not the should name it different. Shoulder (DOF 2)
    const lift = new THREE.Mesh(new THREE.CylinderGeometry(0.3, 0.3, LIFT_HEIGHT, 32), material);
    lift.position.set(0, LIFT_HEIGHT/2, 0);
    swingJoint.add(lift);

    // Linear lift joint
    const liftJoint = new THREE.Object3D();
    liftJoint.position.set(0, 0, 0)
    lift.add(liftJoint);

    // Elbow (DOF 3)
    const elbow = new THREE.Mesh(new THREE.BoxGeometry(0.5, ELBOW_LENGTH, 0.5), material);
    elbow.position.set(ELBOW_LENGTH/2, 0, 0);
    elbow.rotation.set(0, 0, Math.PI/2);
    liftJoint.add(elbow);

    const elbowJoint = new THREE.Object3D();
    elbowJoint.position.set(0, -ELBOW_LENGTH/2, 0)
    elbow.add(elbowJoint);

    const wristStem = new THREE.Mesh(new THREE.CylinderGeometry(0.2, 0.2, 0.9, 32), material);
    wristStem.position.set(-0.2, 0, 0);
    wristStem.rotation.set(0, 0, Math.PI/2);
    elbowJoint.add(wristStem);

    // Wrist 
    const wrist = new THREE.Mesh(new THREE.BoxGeometry(0.4, WRIST_LENGTH, 0.4), material);
    wrist.position.set(-0.45, -WRIST_LENGTH/2, 0);
    elbowJoint.add(wrist);

    const wristJoint = new THREE.Object3D();
    wristJoint.position.set(0, -WRIST_LENGTH/2, 0);
    wrist.add(wristJoint);

    const gripperStem = new THREE.Mesh(new THREE.CylinderGeometry(0.15, 0.15, 0.8, 32), material);
    gripperStem.position.set(-0.2, 0, 0);
    gripperStem.rotation.set(0, 0, Math.PI/2);
    wristJoint.add(gripperStem);

    const gripperBase = new THREE.Mesh(new THREE.BoxGeometry(GRIPPER_BASE_LENGTH, 0.2, GRIPPER_BASE_WIDTH), material);
    gripperBase.position.set(-GRIPPER_BASE_LENGTH/2, 0.3, 0);
    gripperStem.add(gripperBase);

    let openMM = 0.4;

    const gripperL = new THREE.Mesh(new THREE.BoxGeometry(GRIPPERS_LENGTH, 0.2, GRIPPERS_WIDTH), material);
    gripperL.position.set(-GRIPPER_BASE_LENGTH/2-GRIPPERS_LENGTH/2, 0, -(GRIPPERS_WIDTH+openMM)/2);
    gripperBase.add(gripperL);

    const gripperR = new THREE.Mesh(new THREE.BoxGeometry(GRIPPERS_LENGTH, 0.2, GRIPPERS_WIDTH), material);
    gripperR.position.set(-GRIPPER_BASE_LENGTH/2-GRIPPERS_LENGTH/2, 0, (GRIPPERS_WIDTH+openMM)/2);
    gripperBase.add(gripperR);

    // Animation loop for rendering
    const animate = () => {
      requestAnimationFrame(animate);

      const state = robotStateRef.current;
      if (state) {
        swingJoint.rotation.y = THREE.MathUtils.degToRad(state.swing_rotation_deg);
        liftJoint.position.y = state.lift_elevation_mm / 1000; // Convert mm to meters
        elbowJoint.rotation.x = THREE.MathUtils.degToRad(state.elbow_rotation_deg);
        wristJoint.rotation.x = THREE.MathUtils.degToRad(state.wrist_rotation_deg);
      }

      renderer.render(scene, camera);
    };
    animate();

    // Clean up when component unmounts
    return () => {
      refContainer.current?.removeChild(renderer.domElement);
      renderer.dispose();
    };
  }, []);
  return (
    <div ref={refContainer}></div>

  );
}

export default RobotVisualization