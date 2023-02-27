import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
import "@elements/entry/asset/_common/bg";

@customElement("radio-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
               .container {
                    display: block;
                    position: relative;
                    padding-left: 35px;
                    margin-bottom: 12px;
                    cursor: pointer;
                    font-size: 22px;
                    -webkit-user-select: none;
                    -moz-user-select: none;
                    -ms-user-select: none;
                    user-select: none;
                 }

                /* Hide the browser's default radio button */
                .container input {
                    position: absolute;
                    opacity: 0;
                    cursor: pointer;
                }

                /* Create a custom radio button */
                .checkmark {
                    position: absolute;
                    top: 0;
                    left: 0;
                    height: 25px;
                    width: 25px;
                    background-color: #eee;
                    border-radius: 50%;
                }

                /* On mouse-over, add a grey background color */
                .container:hover input ~ .checkmark {
                    background-color: #ccc;
                }

                /* When the radio button is checked, add a blue background */
                .container input:checked ~ .checkmark {
                    background-color: #2196F3;
                }

                /* Create the indicator (the dot/circle - hidden when not checked) */
                .checkmark:after {
                    content: "";
                    position: absolute;
                    display: none;
                }

                /* Show the indicator (dot/circle) when checked */
                .container input:checked ~ .checkmark:after {
                    display: block;
                }

                /* Style the indicator (dot/circle) */
                .container .checkmark:after {
                    top: 9px;
                    left: 9px;
                    width: 8px;
                    height: 8px;
                    border-radius: 50%;
                    background: white;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    checked: boolean = false;

    render() {
        return html`
           <label class="container">
                <input type="radio" .checked=${this.checked} name="radio">
                <span class="checkmark"></span>
            </label>
        `;
    }
}