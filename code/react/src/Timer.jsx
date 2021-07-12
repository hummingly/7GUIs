import { useLayoutEffect, useState } from "react";

export const Timer = () => {
    // time in milliseconds
    const [elapsedTime, setElapsedTime] = useState(0);
    // const [duration, setDuration] = useState(15000);
    const [duration, setDuration] = useState(0);

    useLayoutEffect(() => {
        const t0 = performance.now();
        let timerHandle = undefined;

        const updateElapsedTime = () => {
            timerHandle = undefined;
            const t1 = performance.now();
            setElapsedTime((t) => t + (t1 - t0));
        };

        if (elapsedTime < duration) {
            timerHandle = setTimeout(updateElapsedTime, 100);
        } else {
            setElapsedTime(duration);
        }

        return () => {
            window.clearTimeout(timerHandle);
            updateElapsedTime();
        };
    }, [elapsedTime, duration]);

    return (
        <section>
            <label>
                Elapsed Time: {(elapsedTime / 1000).toFixed(1)}s<br />
                <meter value={elapsedTime} min={0} max={duration}></meter>
            </label>
            <label>
                Duration: {(duration / 1000).toFixed(1)}s<br />
                <input
                    type='range'
                    min={0}
                    max={30000}
                    step={100}
                    value={duration}
                    onChange={(e) => setDuration(e.currentTarget.valueAsNumber)}
                />
            </label>
            <button onClick={() => setElapsedTime(0)}>Reset Timer</button>
        </section>
    );
};
