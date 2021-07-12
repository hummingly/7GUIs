import { useReducer, useState } from "react";

export const Crud = () => {
    const [state, dispatch] = useReducer(reducer, init());
    const [query, setQuery] = useState("");
    const [name, setName] = useState("");
    const [surname, setSurname] = useState("");
    const [selection, setSelection] = useState(state.users.length > 0 ? state.users[0].id : "");

    const handleCreate = () => {
        dispatch({ type: CREATE, payload: { name, surname } });
        setName("");
        setSurname("");
    };

    const handleUpdate = () => {
        dispatch({ type: UPDATE, payload: { id: selection, name, surname } });
        setName("");
        setSurname("");
    };

    const handleDelete = () => {
        dispatch({ type: DELETE, payload: { id: selection } });
        setName("");
        setSurname("");
    };

    return (
        <section>
            <label>
                Filter prefix:
                <input
                    type='text'
                    value={query}
                    onChange={(e) => setQuery(e.currentTarget.value)}
                />
            </label>
            <div>
                <select size={10} onChange={(e) => setSelection(e.currentTarget.value)}>
                    {state.users
                        .filter((u) => filterUser(query.toLowerCase(), u))
                        .map((u) => {
                            return (
                                <option key={u.id} value={u.id}>
                                    {u.surname}, {u.name}
                                </option>
                            );
                        })}
                </select>
                <div>
                    <label>
                        Name:
                        <input
                            type='text'
                            value={name}
                            onChange={(e) => setName(e.currentTarget.value)}
                        />
                    </label>
                    <label>
                        Surname:
                        <input
                            type='text'
                            value={surname}
                            onChange={(e) => setSurname(e.currentTarget.value)}
                        />
                    </label>
                </div>
            </div>
            <div>
                <button onClick={handleCreate}>Create</button>
                <button onClick={handleUpdate}>Update</button>
                <button onClick={handleDelete}>Delete</button>
            </div>
        </section>
    );
};

let ID_COUNTER = 0;

class Db {
    users = [];

    constructor() {
        this.create("Hans", "Emil");
        this.create("Max", "Mustermann");
        this.create("Roman", "Tisch");
    }

    create(name, surname) {
        const id = ID_COUNTER;
        ID_COUNTER += 1;
        this.users.push(new User(id.toString(), name, surname));
    }

    update(id, name, surname) {
        const user = this.users.find((u) => u.id === id);
        if (user) {
            user.name = name;
            user.surname = surname;
        }
    }

    delete(id) {
        const index = this.users.findIndex((u) => u.id === id);
        this.users.splice(index, 1);
    }
}

class User {
    id;
    name;
    surname;

    constructor(id, name, surname) {
        this.id = id;
        this.name = name;
        this.surname = surname;
    }
}

const CREATE = "create";
const UPDATE = "update";
const DELETE = "delete";

const reducer = (state, action) => {
    switch (action.type) {
        case CREATE: {
            const { name, surname } = action.payload;
            state.db.create(name, surname);
            break;
        }

        case UPDATE: {
            const { id, name, surname } = action.payload;
            state.db.update(id, name, surname);
            break;
        }

        case DELETE: {
            const { id } = action.payload;
            state.db.delete(id);
            break;
        }

        default:
            return state;
    }
    return { ...state, users: state.db.users.slice() };
};

const init = () => {
    const db = new Db();
    const users = db.users.slice();
    return { db, users };
};

function filterUser(query, user) {
    if (query.length === 0) {
        return true;
    }
    const { name, surname } = user;
    return name.toLowerCase().includes(query) || surname.toLowerCase().includes(query);
}
