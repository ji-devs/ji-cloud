import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";

// <password-strength strength="2" />

export type Strength = "none" | "weak" | "average" | "strong";

@customElement("password-strength")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .wrapper {
                    width: 257px;
                    height: 8px;
                    border-radius: 4px;
                    position: relative;
                    margin-bottom: 12px;
                }
                .inner {
                    position: absolute;
                    border-radius: 4px;
                    height: 8px;
                }
                .strength-weak {
                    background-color: #f84f57;
                    width: 33%;
                }
                .strength-average {
                    width: 66%;
                    background-color: #fccd63;
                }
                .strength-strong {
                    width: 100%;
                    background-color: #42cc7a;
                }
            `,
        ];
    }

    @property()
    strength: Strength = "none";

    render() {
        const { strength } = this;

        const className =
            strength === "weak"
                ? "strength-weak"
                : strength === "average"
                ? "strength-average"
                : strength === "strong"
                ? "strength-strong"
                : "";

        return html`
            <div class="wrapper ${className}">
                <div class="inner"></div>
            </div>
        `;
    }
}
