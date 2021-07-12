import ReactDOM from "react-dom";
import { Counter } from "./Counter";
import { TemperatureConverter } from "./TemperatureConverter";
import { FlightBooker } from "./FlightBooker";
import { Timer } from "./Timer";
import { Crud } from "./Crud";
import { CircleDrawer } from "./CircleDrawer";
import { Cells } from "./Cells";

const App = () => {
    return (
        <main>
            <h1>React 7GUIs</h1>
            <Counter />
            <TemperatureConverter />
            <FlightBooker />
            <Timer />
            <Crud />
            <CircleDrawer />
            <Cells />
        </main>
    );
};

ReactDOM.render(<App />, document.getElementById("root"));
