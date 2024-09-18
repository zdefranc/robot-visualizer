// utils/socket.ts
import { io, Socket } from 'socket.io-client';

// Connect to your server at localhost (or your specific server address)
const socket: Socket = io('ws://127.0.0.1:3000'); // Adjust URL if necessary

export default socket;
