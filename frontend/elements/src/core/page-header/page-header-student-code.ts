import { LitElement, html, css, customElement } from "lit-element";
import "@elements/core/overlays/anchored-overlay";

const STR_STUDENT_CODE = "Student Code";

@customElement("page-header-student-code")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    width: 124px;
                    position: relative;
                    display: grid;
                }
                .img-wrapper {
                    display: grid;
                    justify-content: center;
                    align-items: end;
                    height: 68px;
                }
                @media (min-width: 1920px) {
                    .img-wrapper {
                        height: 88px;
                    }
                }
                .overlay {
                    position: absolute;
                    background-color: var(--light-blue-3);
                    color: var(--dark-gray-6);
                    text-align: center;
                    width: 124px;
                    line-height: 34px;
                    right: 0;
                    bottom: 0;
                    transform: translateY(100%);
                    font-size: 13px;
                    font-weight: 600;
                    text-decoration: none;
                    border-radius: 0 0 12px 12px;
                }
            `,
        ];
    }

    render() {
        return html`
            <a href="/kids">
                <div class="img-wrapper">
                    <img-ui path="core/page-header/kids.svg"></img-ui>
                </div>
                <span class="overlay">${STR_STUDENT_CODE}</span>
            </a>
        `;
    }
}
