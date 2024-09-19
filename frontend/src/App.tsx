
import { Robot } from "./components/Robot"
import { useState, useEffect, useCallback } from "react"




function App() {
  const [count, setCount] = useState<number | null>(null)

  return (
    <div>
      <Robot />
    </div>
  );
}

export default App
