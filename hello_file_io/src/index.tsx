import React, { useRef } from "react";
import ReactDOM from "react-dom/client";

const App = () => {
    const inputRef = useRef<HTMLInputElement>(null);
    return (
        <>
            {/* button のイベントを input に流して input の見た目をカスタマイズする*/}
            <button onClick={() => { inputRef.current?.click() }}>ファイルアップロード</button >
            <input
                hidden
                ref={inputRef}
                type="file"
                accept=".json"
                onChange={() => { }}
            />
        </>
    );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
