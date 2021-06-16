import { LitElement, html, css, customElement, property } from "lit-element";

export type mode = 'default' | 'active' | 'success' | 'done';

@customElement("audio-input")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-rows: 24px 320px auto;
                    row-gap: 20px;
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
                :host([mode=default]) .main-content {
                    border-color: var(--light-blue-4);
                    border-style: solid;
                }
                :host([mode=active]) .main-content {
                    border-color: var(--dark-blue-1);
                    border-style: dashed;
                }
                :host([mode=success]) .main-content {
                    border-color: var(--green-4);
                    background-color: var(--green-2);
                    border-style: solid;
                }
                :host([mode=done]) .main-content {
                    border-color: var(--green-4);
                    border-style: solid;
                }
                ::slotted(progress-bar[slot=main-content]) {
                    width: 272px;
                }
                ::slotted(input-file[slot=main-content]) {
                    height: 100%;
                    width: 100%;
                }
                .actions {
                    padding: 0 26px;
                    display: grid;
                    align-items: center;
                    grid-template-columns: 1fr 2fr 1fr;
                }
                ::slotted([slot=main-action]) {
                    grid-column: 2;
                    justify-self: center;
                }
            `,
        ];
    }

    @property({type: String, reflect: true})
    mode: mode = 'default';

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
