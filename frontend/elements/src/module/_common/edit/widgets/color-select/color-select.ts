import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement("color-select")
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                display: grid;
                row-gap: 32px;
                padding-bottom: 32px;
            }
            h2 {
                margin: 0;
                font-size: 16px;
                font-weight: normal;
            }
            .sections {
                display: grid;
                row-gap: 32px;
            }
            hr {
                width: 100%;
                margin: 0;
                background: var(--light-gray-4);
            }
        `];
    }

    @property()
    label: string = "";

    render() {
        return html`
            <h2>${this.label}</h2>
            <div class="sections">
                <slot name="sections"></slot>
            </div>
            <hr>
            <slot name="add-color"></slot>
        `;
    }
}
