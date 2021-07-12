import { memo, useEffect, useRef, useState } from "react";

export const CircleDrawer = () => {
    const drawerRef = useRef(null);
    const inputRef = useRef(null);
    const model = useRef(new DrawerModel());
    const [circles, setCircles] = useState([]);
    const selectionIndex = useRef(-1);
    const [menu, setMenu] = useState({ open: false, x: 0, y: 0 });
    const [slider, setSlider] = useState(30);

    useEffect(() => {
        const handleNotification = (e) => {
            const { id, clientX, clientY } = e.detail;
            const rect = e.currentTarget.getBoundingClientRect();
            const x = clientX - rect.x;
            console.log(e.currentTarget.clientLeft);
            const y = clientY - rect.y - rect.height;
            selectionIndex.current = model.current.circles.findIndex((c) => c.id === id);
            setSlider(model.current.circles[selectionIndex.current].d);
            setMenu({ open: true, x, y });
        };
        drawerRef.current.addEventListener(SHOW_CIRCLE_MENU, handleNotification);
        return () => {
            drawerRef.current.removeEventListener(SHOW_CIRCLE_MENU, handleNotification);
        };
    }, []);

    useEffect(() => {
        if (menu.open) {
            inputRef.current.focus();
        }
    }, [menu.open]);

    const handleUndo = useRef(() => {
        if (model.current.undo()) {
            setCircles(model.current.circles.slice());
        }
    }).current;

    const handleRedo = useRef(() => {
        if (model.current.redo()) {
            setCircles(model.current.circles.slice());
        }
    }).current;

    const handleCreate = useRef((e) => {
        const rect = e.currentTarget.getBoundingClientRect();
        const x = e.clientX - rect.x;
        const y = e.clientY - rect.y;
        const updated = model.current.create(x, y);
        if (updated) {
            setCircles(model.current.circles.slice());
        }
    }).current;

    const handleSlider = useRef((e) => {
        const diameter = e.currentTarget.valueAsNumber;
        setSlider(diameter);
        setCircles((c) => {
            const { id, x, y } = c[selectionIndex.current];
            const updatedCircle = Object.freeze(new CircleModel(id, x, y, diameter));
            c[selectionIndex.current] = updatedCircle;
            return c.slice();
        });
    }).current;

    const handleUpdate = () => {
        const { id, d } = model.current.circles[selectionIndex.current];
        if (d !== slider && model.current.update(id, slider)) {
            setCircles(model.current.circles.slice());
        }
        setMenu((m) => ({ ...m, open: false }));
    };

    const disabledUndo = model.current.historyIndex === -1;
    const disabedRedo = model.current.historyIndex === model.current.history.length - 1;

    return (
        <section id='circle-drawer'>
            <h2>Circle Drawer</h2>
            <div className='circle-drawer-toolbar'>
                <button
                    className='circle-drawer-button'
                    onClick={handleUndo}
                    disabled={disabledUndo}>
                    Undo
                </button>
                <button
                    className='circle-drawer-button'
                    onClick={handleRedo}
                    disabled={disabedRedo}>
                    Redo
                </button>
            </div>
            <div className='circle-drawer-canvas' ref={drawerRef} onClick={handleCreate}>
                {circles.map(({ id, x, y, d }) => (
                    <Circle key={id} id={id} x={x} y={y} d={d} />
                ))}
            </div>
            <ContextMenu open={menu.open} x={menu.x} y={menu.y}>
                <label htmlFor='diameter-slider'>Adjust Diameter</label>
                <input
                    id='diameter-slider'
                    ref={inputRef}
                    type='range'
                    min={2}
                    max={100}
                    value={slider}
                    onChange={handleSlider}
                    onBlur={handleUpdate}
                />
            </ContextMenu>
        </section>
    );
};

const Circle = memo(function Circle({ id, x, y, d }) {
    const diameter = `${d}px`;
    // 2 extra pixel due to border line
    const offset = Math.round((d + 2) / 2);
    return (
        <div
            className='circle-drawer-circle'
            onClick={(e) => {
                const notification = new CustomEvent(SHOW_CIRCLE_MENU, {
                    bubbles: true,
                    detail: { id, clientX: e.clientX, clientY: e.clientY },
                });
                e.currentTarget.dispatchEvent(notification);
                e.stopPropagation();
            }}
            style={{
                width: diameter,
                height: diameter,
                transform: `translate(${x - offset}px,${y - offset}px)`,
            }}
        />
    );
});

const ContextMenu = memo(function ContextMenu({ open, x, y, children }) {
    return (
        <div
            className='circle-drawer-menu'
            data-open={open}
            style={{ transform: `translate(${x}px,${y}px)` }}>
            {children}
        </div>
    );
});

const SHOW_CIRCLE_MENU = "show_circle_menu";

const CREATE = "create";
const UPDATE = "update";

let ID_COUNTER = 0;

class DrawerModel {
    history = [];
    historyIndex = -1;
    circles = [];

    create(x, y) {
        const circle = Object.freeze(new CircleModel(ID_COUNTER, x, y, 30));
        this.circles.push(circle);
        ID_COUNTER += 1;

        this.history.splice(this.historyIndex + 1);
        this.history.push({ type: CREATE, circle });
        this.historyIndex = this.history.length - 1;
        return true;
    }

    update(id, diameter) {
        const index = this.circles.findIndex((c) => c.id === id);
        if (index > -1) {
            const { x, y, d } = this.circles[index];
            if (d === diameter) {
                return false;
            }
            const circle = Object.freeze(new CircleModel(id, x, y, diameter));
            this.history.splice(this.historyIndex + 1);
            this.history.push({ type: UPDATE, circle });
            this.historyIndex = this.history.length - 1;
            this.circles[index] = circle;
            return true;
        }
        return false;
    }

    undo() {
        const change = this.history[this.historyIndex];
        this.historyIndex -= 1;
        const id = change.circle.id;
        const circleIndex = this.circles.findIndex((c) => id === c.id);
        if (change.type === CREATE) {
            this.circles.splice(circleIndex, 1);
            return true;
        } else if (change.type === UPDATE) {
            for (let i = this.historyIndex; i >= 0; i--) {
                const c = this.history[i];
                if (id === c.circle.id) {
                    this.circles[circleIndex] = c.circle;
                    return true;
                }
            }
        }
        return false;
    }

    redo() {
        this.historyIndex += 1;
        const change = this.history[this.historyIndex];
        if (change.type === CREATE) {
            this.circles.push(change.circle);
            return true;
        } else if (change.type === UPDATE) {
            const id = change.circle.id;
            const index = this.circles.findIndex((c) => id === c.id);
            this.circles[index] = change.circle;
            return true;
        }
        return false;
    }
}

class CircleModel {
    constructor(id, x, y, d) {
        this.id = id;
        this.x = x;
        this.y = y;
        this.d = d;
    }
}
