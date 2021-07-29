import { LitElement, html, css, customElement, property } from "lit-element";

export type Kind = "students" | "embed" | "copy";

const STR_LABEL_LOOKUP: {
    [key in Kind]: string
} = {
    ['students']: "Share with students",
    ['embed']: "Embed this JIG",
    ['copy']: "Copy URL",
};


@customElement("jig-play-sidebar-share-option")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: auto 1fr auto;
                    align-items: center;
                    column-gap: 12px;
                    padding: 8px;
                    cursor: pointer;
                }
                :host(:hover) {
                    background-color: var(--light-blue-1);
                }
                .help {
                    height: 24px;
                    width: 24px;
                    display: inline-grid;
                    place-content: center;
                    color: var(--main-blue);
                    background-color: var(--light-blue-2);
                    border-radius: 50%;
                }
            `,
        ];
    }

    @property()
    kind: Kind = "students";

    render() {
        return html`
            <slot slot="back" name="back"></slot>
            <slot slot="close" name="close"></slot>
            <img-ui path="entry/jig/play/sidebar/share-${this.kind}.svg"></img-ui>
            <span class="label">${STR_LABEL_LOOKUP[this.kind]}</span>
            <span class="help">?</span>
        `;
    }
}
