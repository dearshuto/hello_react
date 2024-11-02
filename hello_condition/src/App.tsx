import { useState } from "react";
import "./App.css";

interface Args {
  count: number;
}

const MessageComponent = (args: Args) => {
  if (args.count % 2 === 0) {
    return <p>count: {args.count} even</p>;
  } else {
    return <p>count: {args.count} odd</p>;
  }
};

function App() {
  let [count, setCount] = useState(0);

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={() => setCount(count + 1)}>Click</button>
        <MessageComponent count={count} />
      </header>
    </div>
  );
}

export default App;
