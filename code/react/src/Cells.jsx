import { memo, useState } from "react";

export const Cells = () => {
    return (
        <section>
            <h2>Cells</h2>
            <table role='grid' className='cells-grid'>
                <Header />
            </table>
        </section>
    );
};

const Row = ({ index, children }) => {
    return (
        <tr className='cells-row'>
            <th scope='row' className='cells-row-cell'>
                {index}
            </th>
            {children}
        </tr>
    );
};

const Header = memo(function Header() {
    const letters = [
        "A",
        "B",
        "C",
        "D",
        "E",
        "F",
        "G",
        "H",
        "I",
        "J",
        "K",
        "L",
        "M",
        "N",
        "O",
        "P",
        "Q",
        "R",
        "S",
        "T",
        "U",
        "V",
        "W",
        "X",
        "Y",
        "Z",
    ];
    return (
        <tr className='cells-row'>
            <th scope='col' className='cells-row-cell' />
            {letters.map((l) => (
                <th scope='col' className='cells-row-cell'>
                    {l}
                </th>
            ))}
        </tr>
    );
});

const Cell = ({ isEditing, index }) => {
    const [text, setText] = useState("");

    return (
        <td aria-readonly={!isEditing} className='cells-row-cell'>
            <input readOnly={!isEditing} />
        </td>
    );
};
