import { html, css } from "lit-element";
import { Letter, LETTER_INFO, LetterInfo, niqquds } from "./data";

export const styles = css``;

export function render(letter: Letter, onClick: (char: string) => void) {
    const info = LETTER_INFO[letter];

    return html`
        <button class="${letter}" @click="${() => onClick(info.char)}">
            <span class="tooltip">${info.name}</span>
            ${info.char}
        </button>
    `;
}
