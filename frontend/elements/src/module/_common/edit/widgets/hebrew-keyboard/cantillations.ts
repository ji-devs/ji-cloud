import { css, html } from "lit-element";
import { Cantillation, cantillations, CANTILLATION_INFO } from "./data";
import "@elements/core/overlays/anchored-overlay";

const STR_TAAMAI_MIKRA = "Taamai mikra";

export const cantillationsStyles = css`

`;


export function renderCantillations(onClick: (char: string) => void, toggleOpen: () => void) {
    return html`
        <anchored-overlay id="cantillations" positionY="center" positionX="left-out">
            <div slot="overlay">
                ${cantillations.map(cantillation => {
                    return renderCantillation(cantillation, onClick);
                })}
            </div>
            <button slot="anchor" @click="${toggleOpen}">
                <span class="tooltip">${STR_TAAMAI_MIKRA}</span>
                טעמי מקרא
            </button>
        </anchored-overlay>
    `;
}

function renderCantillation(cantillation: Cantillation, onClick: (char: string) => void) {
    const info = (CANTILLATION_INFO as any)[cantillation];

    return html`
        <button class="image-button" @click="${() => onClick(info.char)}">
            <span class="tooltip">${info.name}</span>
            <span class="letter-placeholder"></span>
            <img-ui path="module/_common/edit/widgets/hebrew-keyboard/cantillation-${cantillation}.svg"></img-ui>
        </button>
    `;
}
