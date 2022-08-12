import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("module-sidebar-body")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    flex-direction: column;
                    overflow: auto;
                    /* also set height to allow descendants to take full height, feels hacky */
                    height: 100%;
                    min-height: 100%;
                }

                :host([dark]) {
                    background-color: #e9eff8;
                }

                section {
                    box-sizing: border-box;
                    padding: 0 20px;
                    flex: 1;
                }

                ::slotted([slot=action]) {
                    padding: 18px 0;
                    border-top: solid 1px var(--light-blue-4);
                    width: 100%;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    dark: boolean = false;

    render() {
        return html`
            <section>
                <slot></slot>
            </section>
            <slot name="action"></slot>`;
    }
}
