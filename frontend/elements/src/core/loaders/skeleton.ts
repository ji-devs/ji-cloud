import { LitElement, html, css, customElement } from "lit-element";

@customElement("loader-skeleton")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    background-color: var(--light-blue-2);
                    display: inline-block;
                }
                div {
                    height: 100%;
                    width: 100%;
                    background-image: linear-gradient(90deg, var(--light-blue-2), #fff, var(--light-blue-2));
                    animation: loading 1.2s infinite;
                    background-position: 200% 0;
                    background-size: 200% 100%;
                    background-repeat: no-repeat;
                }
                @keyframes loading {
                    to {
                        background-position: -200% 0;
                    }
                }
            `,
        ];
    }

    render() {
        return html`
            <div></div>
        `;
    }
}
