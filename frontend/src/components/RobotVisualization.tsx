import * as THREE from 'three';
import { JointState, RobotState } from '../types/RobotTypes';

import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

import { useEffect, useRef } from "react";

const BASE_HEIGHT = 0.3; // Important for IK
const BASE_RADIUS = 0.6; 

const LIFT_HEIGHT = 4; 
const LIFT_RADIUS = 0.3;

const ELBOW_LENGTH = 2; // Important for IK
const ELBOW_WIDTH = 0.3;
const ELBOW_DEPTH = 0.5;

const WRIST_LENGTH = 1; // Important for IK
const WRIST_WIDTH = 0.25;
const WRIST_DEPTH = 0.35;

const WRIST_STEM_LENGTH = WRIST_WIDTH+ELBOW_WIDTH;
const WRIST_STEM_RADIUS = 0.2;

const GRIPPER_BASE_LENGTH = 0.6;
const GRIPPER_BASE_WIDTH = 0.1;
const GRIPPER_BASE_DEPTH = 0.3;

const GRIPPER_STEM_LENGTH = 0.6;
const GRIPPER_STEM_RADIUS = 0.15;

const GRIPPERS_HEIGHT = 0.05;
const GRIPPERS_WIDTH = GRIPPER_BASE_DEPTH;
const GRIPPERS_LENGTH = GRIPPER_BASE_WIDTH+0.2;

const GRIPPER_STATIC_OFFSET = -0.1;

const LIFT_ZERO = -LIFT_HEIGHT/2+GRIPPER_STEM_LENGTH+GRIPPERS_LENGTH-GRIPPER_BASE_WIDTH;


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
    const camera = new THREE.PerspectiveCamera(75, window.innerWidth/2 / window.innerHeight, 0.1, 1000);
    camera.position.z = 10;
    camera.position.y = 3;
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

    // Provides an orbital camera.
    const controls = new OrbitControls(camera, renderer.domElement);
    
    // Add light to the scene
    const light = new THREE.DirectionalLight(0xffffff, 1);
    light.position.set(5, 10, 7.5);
    scene.add(light);

    // Create the robot arm parts (cylinders to represent each segment)
    const material = new THREE.MeshStandardMaterial({ color: 0x0077ff });

    // Base
    const base = new THREE.Mesh(new THREE.CylinderGeometry(LIFT_RADIUS, BASE_RADIUS, BASE_HEIGHT, 32), material);
    base.position.set(0, BASE_HEIGHT/2, 0);
    scene.add(base);
    
    // swing joint (DOF 1)
    const swingJoint = new THREE.Object3D();
    swingJoint.position.set(0, 0, 0)
    base.add(swingJoint);

    // Lift. 
    const lift = new THREE.Mesh(new THREE.CylinderGeometry(LIFT_RADIUS, LIFT_RADIUS, LIFT_HEIGHT, 32), material);
    lift.position.set(0, LIFT_HEIGHT/2, 0);
    lift.rotation.set(0, 0, 0);
    swingJoint.add(lift);

    // Linear lift joint (DOF 2)
    const liftJoint = new THREE.Object3D();
    // Come back to this so the arm isnt in the floor at zero!
    liftJoint.position.set(0, -LIFT_HEIGHT/2, 0)
    lift.add(liftJoint);

    // Elbow 
    const elbow = new THREE.Mesh(new THREE.BoxGeometry(ELBOW_WIDTH, ELBOW_LENGTH, ELBOW_DEPTH), material);
    elbow.position.set(ELBOW_LENGTH/2, 0, 0);
    elbow.rotation.set(0, 0, Math.PI/2);
    liftJoint.add(elbow);

    // Elbow joint (DOF 3)
    const elbowJoint = new THREE.Object3D();
    elbowJoint.position.set(0, -ELBOW_LENGTH/2, 0)
    elbow.add(elbowJoint);

    const wristStem = new THREE.Mesh(new THREE.CylinderGeometry(WRIST_STEM_RADIUS, WRIST_STEM_RADIUS, WRIST_STEM_LENGTH, 32), material);
    wristStem.position.set(-(WRIST_STEM_LENGTH-ELBOW_WIDTH)/2, 0, 0);
    wristStem.rotation.set(0, 0, Math.PI/2);
    elbowJoint.add(wristStem);

    // Wrist 
    const wrist = new THREE.Mesh(new THREE.BoxGeometry(WRIST_WIDTH, WRIST_LENGTH, WRIST_DEPTH), material);
    wrist.position.set(-(WRIST_STEM_LENGTH-(ELBOW_WIDTH+WRIST_WIDTH)/2), -WRIST_LENGTH/2, 0);
    elbowJoint.add(wrist);

    // Wrist joint (DOF 3)
    const wristJoint = new THREE.Object3D();
    wristJoint.position.set(0, -WRIST_LENGTH/2, 0);
    wrist.add(wristJoint);

    const gripperStem = new THREE.Mesh(new THREE.CylinderGeometry(GRIPPER_STEM_RADIUS, GRIPPER_STEM_RADIUS, GRIPPER_STEM_LENGTH, 32), material);
    gripperStem.position.set(-(GRIPPER_STEM_LENGTH-WRIST_WIDTH)/2, 0, 0);
    gripperStem.rotation.set(0, 0, Math.PI/2);
    wristJoint.add(gripperStem);

    const gripperBase = new THREE.Mesh(new THREE.BoxGeometry(GRIPPER_BASE_WIDTH, GRIPPER_BASE_LENGTH, GRIPPER_BASE_DEPTH), material);
    gripperBase.position.set(-GRIPPER_BASE_LENGTH/2, (GRIPPER_STEM_LENGTH-GRIPPER_BASE_WIDTH)/2, 0);
    gripperBase.rotation.set(0, 0, Math.PI/2);
    gripperStem.add(gripperBase);

    const gripperStatic = new THREE.Mesh(new THREE.BoxGeometry(GRIPPERS_LENGTH, GRIPPERS_HEIGHT, GRIPPERS_WIDTH), material);
    gripperStatic.position.set((GRIPPERS_LENGTH-GRIPPER_BASE_WIDTH)/2, GRIPPER_STATIC_OFFSET, 0);
    // gripperBase.rotation.set(0, 0, 0);
    gripperBase.add(gripperStatic);

    const gripperDynamic = new THREE.Mesh(new THREE.BoxGeometry(GRIPPERS_LENGTH, GRIPPERS_HEIGHT, GRIPPERS_WIDTH), material);
    gripperDynamic.position.set(0, 0, 0);
    gripperStatic.add(gripperDynamic);

    // Animation loop for rendering
    const animate = () => {
      requestAnimationFrame(animate);

      controls.update();

      const state = robotStateRef.current;
      if (state) {
        swingJoint.rotation.y = THREE.MathUtils.degToRad(state.joint_state.swing_rotation_deg);
        liftJoint.position.y = state.joint_state.lift_elevation_mm / 1000 + LIFT_ZERO; // Convert mm to meters
        elbowJoint.rotation.x = THREE.MathUtils.degToRad(state.joint_state.elbow_rotation_deg);
        wristJoint.rotation.x = THREE.MathUtils.degToRad(state.joint_state.wrist_rotation_deg);
        gripperDynamic.position.y = (state.joint_state.gripper_open_mm/1000+GRIPPERS_HEIGHT);

        base.position.set(state.base_state.x, state.base_state.z + BASE_HEIGHT/2, - state.base_state.y)
        base.rotation.y = THREE.MathUtils.degToRad(state.base_state.theta);
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