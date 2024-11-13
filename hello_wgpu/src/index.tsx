import React, { useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom/client";

// rust で書いたレンダラ
// React から呼び出すには js ライブラリーとして package.json に登録する
import init from "renderer";

const DesignView = () => {
    const canvasRef = useRef(null);

    useEffect(() => {
        let mounted = true;

        (async () => {
            const wasm = await init();
            if (mounted) {
                if (canvasRef.current) {
                    const handle = wasm.run_on_canvas(canvasRef.current);
                    wasm.emit_user_event(handle);
                }
            }
        })();

        return () => {
            mounted = false;
        };
    }, []);

    return <canvas id="canvas" ref={canvasRef} width={1280} height={960}></canvas>
}

const App = () => {
    const [visible, setVisible] = useState(true);

    return <>
        <button onClick={() => setVisible(!visible)}>{visible ? "Hide" : "Show"}</button>
        <>{visible ? <DesignView /> : <></>}</>
    </>
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode >,
);
