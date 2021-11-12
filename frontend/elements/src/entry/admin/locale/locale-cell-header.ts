import { LitElement, html, css, customElement, property } from "lit-element";

export type FilterOption = [string, boolean];

@customElement("locale-cell-header")
export class _ extends LitElement {
    @property()
    public label: string = "";

    @property({ type: Boolean, reflect: true })
    public adminOnly: boolean = false;

    static get styles() {
        return [
            css`
                :host {
                    border: solid white 2px;
                    display: flex;
                    flex-direction: column;
                    justify-content: flex-end;
                    row-gap: 4px;
                    align-items: flex-end;
                }

                .main-section {
                    background-color: #4472c4;
                    color: white;
                    text-align: left;
                    padding: 10px 10px;
                    font-size: 0.8em;
                    min-height: 18px;
                    align-self: stretch;
                }
                :host([adminonly]) .main-section {
                    background-color: #afabab;
                }
            `,
        ];
    }

    render() {
        return html`
            <slot name="actions"></slot>
            <span class="main-section">${this.label}</span>
        `;
    }
}
