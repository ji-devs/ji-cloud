import { css, html } from 'lit-element';
import { Niqqud, NIQQUD_INFO } from './data';

export const styles = css`
`;

export function render(niqqud: Niqqud, onClick: (char: string) => void) {
    const info = (NIQQUD_INFO as any)[niqqud];

    return html`
        <button class="image-button" @click="${() => onClick(info.char)}">
            <span class="tooltip">${info.name}</span>
            <span class="letter-placeholder"></span>
            <img-ui path="module/_common/edit/widgets/hebrew-keyboard/niqqud-${niqqud}.svg"></img-ui>
        </button>
    `;
}
