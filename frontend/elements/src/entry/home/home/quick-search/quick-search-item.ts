import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('home-quick-search-item')
export class _ extends LitElement {
    static get styles() {
        return [css`
            a {
                display: grid;
                row-gap: 3px;
                width: 290px;
                height: 270px;
                place-content: center;
                justify-items: center;
                text-decoration: none;
                cursor: pointer;
            }
            a:hover {
                background-color: var(--light-gray-1);
            }
            .image-wrapper {
                height: 142px;
                width: 142px;
                border-radius: 50%;
                background-color: #ffffff;
                display: grid;
                place-content: center;
            }
            ::slotted(img-ui) {
                height: 100%;
                width: 100%;
            }
            ::slotted([slot=title]) {
                font-size: 18px;
                font-weight: bold;
                color: var(--dark-gray-6);
                margin: 0;
            }
            ::slotted([slot=subtitle]) {
                font-size: 16px;
                font-weight: normal;
                color: var(--dark-gray-6);
                margin: 0;
            }
        `];
    }

    @property()
    href: string = "";

    render() {
        return html`
            <a href="${this.href}">
                <div class="image-wrapper">
                    <slot name="image"></slot>
                </div>
                <slot name="title"></slot>
                <slot name="subtitle"></slot>
            </a>
        `;
    }
}
