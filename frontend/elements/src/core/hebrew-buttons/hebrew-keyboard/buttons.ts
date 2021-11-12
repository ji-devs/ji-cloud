import { html, css } from "lit-element";

const STR_DELETE = "Erase";
export const deleteStyles = css``;

export function renderDelete(onClick: () => void) {
    return html`
        <button @click="${onClick}">
            <span class="tooltip">${STR_DELETE}</span>
            <img-ui
                path="module/_common/edit/widgets/hebrew-keyboard/delete.svg"
            ></img-ui>
        </button>
    `;
}

const STR_ENTER = "Enter";
export const enderStyles = css`
    .enter-wrapper {
        position: relative;
    }
    .enter {
        position: absolute;
        clip-path: path(
            "M 7 0 L 41 0 C 48 0 48 0 48 7 L 48 65 C 48 72 48 72 41 72 L 13 72 C 6 72 6 72 6 65 T 6 36 C 6 32 6 32 3 32 L 4 32 C 0 32 0 32 0 28 L 0 7 C 0 0 0 0 7 0"
        );
        background-color: var(--light-blue-5);
        border: 0;
        z-index: 3;
        height: 74px;
        width: 48px;
        display: grid;
        justify-content: start;
        align-content: end;
    }
    .enter img-ui {
        margin: 6px;
        height: 12px;
    }
    .enter:hover + .tooltip {
        display: block;
    }
`;

export function renderEnder(onClick: (char: string) => void) {
    return html`
        <div class="enter-wrapper">
            <button @click="${() => onClick("\n")}" class="enter">
                <img-ui
                    path="module/_common/edit/widgets/hebrew-keyboard/enter.svg"
                ></img-ui>
            </button>
            <span class="tooltip">${STR_ENTER}</span>
        </div>
    `;
}

const STR_SPACE = "Space";
export const spaceStyles = css``;

export function renderSpace(onClick: (char: string) => void) {
    return html`
        <button @click="${() => onClick(" ")}">
            <span class="tooltip">${STR_SPACE}</span>
            רווח
        </button>
    `;
}
