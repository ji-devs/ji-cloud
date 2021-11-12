import { LitElement, html, css, customElement } from "lit-element";

@customElement("locale-select-columns")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: 150px 50px 150px 50px;
                    grid-gap: 10px;
                }
                hr {
                    grid-column: 1 / -1;
                    width: 100%;
                }
                header {
                    grid-column: 1 / -1;
                    text-align: center;
                }
                ul {
                    list-style: none;
                    padding: 0;
                    margin: 0;
                }
                .actions {
                    grid-column: 1 / -1;
                    display: flex;
                    justify-content: flex-end;
                    column-gap: 10px;
                }
            `,
        ];
    }

    render() {
        return html`
            <header>Select Fields to Display</header>
            <hr />
            <ul class="columns-hidden">
                <slot name="hidden-columns"></slot>
            </ul>
            <div>
                <slot name="move-actions"></slot>
            </div>
            <ul class="columns-visible">
                <slot name="visible-columns"></slot>
            </ul>
            <div>
                <slot name="sort-actions"></slot>
            </div>
            <hr />
            <div class="actions">
                <slot name="main-actions"></slot>
            </div>
        `;
    }
}
