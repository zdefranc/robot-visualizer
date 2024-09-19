
import { Robot } from "./components/Robot"
import { useState, useEffect, useCallback } from "react"




function App() {
  const [count, setCount] = useState<number | null>(null)

  //Calls useEffect when the specified value changes
  // useEffect(() => {
  //   console.log('mounting')
  //   console.log("Users: ", users)

  //   return () => console.log("unmounting")
  // }, [users])

  // Use for expensive computations to keep from recompiling/rerendering function
  const addTwo = useCallback(() => setCount(prev => prev!=null?prev+2:0), [count])

  return (
    <div>
      <h1>Control Robot</h1>
      <Robot />
    </div>
  );
}

export default App
