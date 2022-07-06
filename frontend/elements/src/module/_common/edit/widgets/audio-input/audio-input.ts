import { LitElement, html, css, customElement, property } from "lit-element";

export type mode = "default" | "active" | "success" | "done";

@customElement("audio-input")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    row-gap: 20px;
                    grid-template-rows: 24px 220px auto;
                }
                @media (min-width: 1920px) {
                    :host {
                        grid-template-rows: 24px 320px auto;
                    }
                }
                .options {
                    display: flex;
                    column-gap: 24px;
                }
                .main-content {
                    border-radius: 16px;
                    background-color: var(--white);
                    border-width: 2px;
                    display: grid;
                    place-items: center;
                }
                :host([mode="default"]) .main-content {
                    border-color: var(--light-blue-4);
                    border-style: solid;
                }
                :host([mode="active"]) .main-content {
                    border-color: var(--dark-blue-1);
                    border-style: dashed;
                }
                :host([mode="success"]) .main-content {
                    border-color: var(--green-4);
                    background-color: var(--green-2);
                    border-style: solid;
                }
                :host([mode="done"]) .main-content {
                    border-color: var(--green-4);
                    border-style: solid;
                }
                ::slotted(progress-bar[slot="main-content"]) {
                    width: 75%;
                }
                ::slotted(input-file[slot="main-content"]) {
                    height: 100%;
                    width: 100%;
                }
                .actions {
                    padding: 0;
                    display: flex;
                    flex-direction: row;
                    justify-content: space-around;
                }
                .actions ::slotted(*) {
                    grid-column: 1;
                    grid-row: 1;
                }
                ::slotted([slot="main-action"]) {
                    justify-self: center;
                }
            `,
        ];
    }

    @property({ type: String, reflect: true })
    mode: mode = "default";

    render() {
        return html`
            <div class="options">
                <slot name="options"></slot>
            </div>
            <div class="main-content">
                <slot name="main-content"></slot>
            </div>
            <div class="actions">
                <slot name="delete"></slot>
                <slot name="main-action"></slot>
            </div>
        `;
    }
}
