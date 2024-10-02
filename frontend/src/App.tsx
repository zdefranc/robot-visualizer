
import { Robot } from "./components/Robot"
import { useState, useEffect, useCallback } from "react"
import styles from '../src/css/Robot.module.css'



function App() {
  const [count, setCount] = useState<number | null>(null)

  return (
    <div className={styles['app']}>
      <Robot />
    </div>
  );
}

export default App
