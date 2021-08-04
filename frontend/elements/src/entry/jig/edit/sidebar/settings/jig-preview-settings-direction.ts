import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";

export type Direction = "ltr" | "rtl"; 

const STR_JIG_DIRECTION = "JIG direction:";

const STR_DIRECTION_LABEL: {
    [key in Direction]: string
} = {
    ['ltr']: "Left to right",
    ['rtl']: "Right to left",
};

@customElement("jig-preview-settings-direction")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .main, label {
                    cursor: pointer;
                    display: flex;
                    column-gap: 8px;
                }
                .pointers {
                    border: solid var(--main-blue) 1px;
                    display: inline-grid;
                    grid-template-columns: 1fr 1fr;
                    width: 48px;
                    height: 24px;
                    border-radius: 8px;
                    overflow: hidden;
                }
                .right-pointer, .left-pointer{
                    color: var(--main-blue);
                    display: inline-grid;
                    place-content: center;
                }
                :host([direction=ltr]) .right-pointer,
                :host([direction=rtl]) .left-pointer {
                    background-color: var(--main-blue);
                    color: white;
                }
            `,
        ];
    }

    @property({ reflect: true })
    direction: Direction = "ltr";

    private toggleDirection() {
        if(this.direction === "ltr")
            this.direction = "rtl";
        else
            this.direction = "ltr";

        this.dispatchEvent(new CustomEvent("custom-direction", {
            detail: {
                direction: this.direction
            }
        }))
    }

    render() {
        return html`
            <div @click=${this.toggleDirection} class="main">
                <div class="pointers">
                    <span class="right-pointer">
                        <i class="fa-solid fa-right"></i>
                    </span>
                    <span class="left-pointer">
                        <i class="fa-solid fa-left"></i>
                    </span>
                </div>
                <label>
                    ${STR_JIG_DIRECTION}
                    <strong>${STR_DIRECTION_LABEL[this.direction]}</strong>
                </label>
            </div>
        `;
    }
}
