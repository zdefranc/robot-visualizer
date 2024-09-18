
import { RobotStateCommand } from "./components/RobotStateCommand"
import { RobotStateDisplay } from "./components/RobotStateDisplay"
import { useState, useEffect, useCallback } from "react"
import MyThree from "./components/MyThree"

import socket from "./utils/socket"



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
      <RobotStateDisplay />
      <RobotStateCommand />
      <MyThree />

    </div>
  );
  // return (
  //   <>
  //     <div>
  //       <h1> 
  //         {count}
  //       </h1>
  //       <button onClick={addTwo}> </button>
  //     </div>
  //     <Heading title={"Helo"} />
  //     <Section>This is my Section.</Section>
  //     <Counter setCount={setCount}>Count is {count}</Counter>
  //     <List<number> items={[2, 1, 1]} render={(item) => <span className="gold">{item}</span>} />
  //   </>
  // )
}

export default App
