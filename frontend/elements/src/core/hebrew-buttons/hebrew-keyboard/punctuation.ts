import { html, css } from "lit-element";
import { Punctuation, PUNCTUATION_INFO } from "./data";

export const styles = css``;

export function render(
    punctuation: Punctuation,
    onClick: (char: string) => void
) {
    const info = PUNCTUATION_INFO[punctuation];

    return html`
        <button class="image-button" @click="${() => onClick(info.char)}">
            <span class="tooltip">${info.name}</span>
            <img-ui
                path="module/_common/edit/widgets/hebrew-keyboard/punctuation-${punctuation}.svg"
            ></img-ui>
        </button>
    `;
}
