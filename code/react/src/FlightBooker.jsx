import { useState } from "react";

const ONE_WAY_FLIGHT = 0;
const RETURN_FLIGHT = 1;

export const FlightBooker = () => {
    const [flightType, setType] = useState(ONE_WAY_FLIGHT);
    const [start, setStart] = useState(new Date());
    const [end, setEnd] = useState(start);

    const canBook = flightType === RETURN_FLIGHT && end < start;

    return (
        <section>
            <select value={flightType} onChange={(e) => setType(parseInt(e.currentTarget.value))}>
                <option value={ONE_WAY_FLIGHT}>one-way flight</option>
                <option value={RETURN_FLIGHT}>return flight</option>
            </select>
            <label>
                Start Date
                <input
                    type='date'
                    value={formatDateValue(start)}
                    onChange={(e) => setStart(e.currentTarget.valueAsDate)}
                    pattern='\d{4}-\d{2}-\d{2}'
                />
            </label>
            <label>
                Return Date
                <input
                    type='date'
                    value={formatDateValue(end)}
                    onChange={(e) => setEnd(e.currentTarget.valueAsDate)}
                    pattern='\d{4}-\d{2}-\d{2}'
                    disabled={flightType === ONE_WAY_FLIGHT}
                />
            </label>
            <button disabled={canBook}>Book</button>
        </section>
    );
};

function formatDateValue(date) {
    const month = date.getMonth();
    const day = date.getDate();
    return `${date.getFullYear()}-${month < 10 ? "0" : ""}${month}-${day < 10 ? "0" : ""}${day}`;
}
