import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";

import init, { InitOutput } from "runtime";

interface Interface {
    add: (lhs: number, rhs: number) => number;
    sub: (lhs: number, rhs: number) => number;
}

const App = (i: Interface) => {
    const [rhs, setRhs] = useState(0);
    const [lhs, setLhs] = useState(0);

    return <>
        <p>Hello World</p>
        <p>{lhs} + {rhs} = {i.add(lhs, rhs)}</p>
        <p>{lhs} - {rhs} = {i.sub(lhs, rhs)}</p>
        <button onClick={() => setLhs(lhs + 1)}>Add Lhs</button>
        <button onClick={() => setRhs(rhs + 1)}>Add Rhs</button>
    </>;
}

const RootComponent = () => {
    const [wasm, setWasm] = useState<InitOutput>();

    useEffect(() => {
        console.log("useEffect()");
        init().then(wasm => {
            console.log("init()");
            setWasm(wasm);
        });
    }, []);

    return <>
        <App add={(x, y) => {
            if (wasm == undefined) {
                return 0;
            }

            return wasm?.add(x, y);
        }}
            sub={(x, y) => {
                if (wasm == undefined) {
                    return 0;
                }

                return wasm?.sub(x, y);
            }} />
    </>;

}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <RootComponent />
    </React.StrictMode>,
);
