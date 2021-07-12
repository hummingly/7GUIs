import { useState } from "react";

export const Counter = () => {
    const [count, setCount] = useState(0);

    return (
        <section>
            <h2>Counter</h2>
            <span>{count}</span>
            <button onClick={() => setCount((c) => c + 1)}>Count</button>
        </section>
    );
};
