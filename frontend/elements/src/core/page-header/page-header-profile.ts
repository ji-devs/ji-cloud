import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('page-header-profile')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: grid;
                grid-template-columns: auto auto;
                column-gap: 16px;
                align-items: center;
            }
            img-ji {
                display: inline-block;
                height: 48px;
                width: 48px;
                border-radius: 50%;
                /* TODO: remove once we have profile images */
                background-color: red;
            }
            .name {
                font-size: 14px;
                font-weight: 500;
                color: var(--dark-blue-8);
            }
        `];
    }

    @property()
    name: string = "";

    render() {
        return html`
            <img-ji></img-ji>
            <span class="name">${this.name}</span>
        `;
    }
}
