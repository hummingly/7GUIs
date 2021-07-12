import { useState } from "react";

export const TemperatureConverter = () => {
    const [celsius, setCelsius] = useState("");
    const [fahrenheit, setFahrenheit] = useState("");

    const handleCelsiusChange = (e) => {
        setCelsius(e.currentTarget.value);

        const value = parseInt(e.currentTarget.value);
        if (!isNaN(value)) {
            setFahrenheit(Math.round(value * 1.8 + 32).toString());
        }
    };

    const handleFahrenheitChange = (e) => {
        setFahrenheit(e.currentTarget.value);

        const value = parseInt(e.currentTarget.value);
        if (!isNaN(value)) {
            setCelsius(Math.round((value - 32) / 1.8).toString());
        }
    };

    return (
        <section>
            <label>
                <input type='text' value={celsius} onChange={handleCelsiusChange} pattern='/d+' />
                Celsius
            </label>
            ⇔
            <label>
                <input
                    type='text'
                    value={fahrenheit}
                    onChange={handleFahrenheitChange}
                    pattern='/d+'
                />
                Fahrenheit
            </label>
        </section>
    );
};
