import {
    LitElement,
    html,
    css,
    customElement,
    property,
    internalProperty
} from "lit-element";

const HUBSPOT_LINK = "https://share.hsforms.com/1pCg45ADPSlCFSiL0NzOYIQ1kii1";

@customElement("beta-tooltip-content")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    font-family: Poppins;
                    color: var(--dark-gray-6);
                }

                h1 {
                    font-size: 20px;
                    font-weight: 500;
                    line-height: 2;
                    text-align: center;
                }
                div.content {
                    margin: 16px 24px;
                    font-size: 16px;
                    line-height: 1.5;
                    text-align: center;
                }
            `,
        ];
    }

    render() {
        return html`
            <h1>You are using the beta version</h1>
            <img-ui path="core/page-header/illustration-beta.png"></img-ui>
            <div class="content">Your feedback will help us improve our platform.</div>
            <button-rect kind="text" color="blue" bold href="${HUBSPOT_LINK}" target="_blank">
                Tell us what you think!
            </button-rect>
        `;
    }
}

