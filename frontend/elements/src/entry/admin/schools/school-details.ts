import {LitElement, html, css, customElement, property} from "lit-element";
import {nothing} from "lit-html";

@customElement("admin-school-details")
export class _ extends LitElement {
    static styles = [
        css`
            :host {
                display: flex;
                flex-direction: column;
                padding: 10px;
                justify-content: center;
                grid-gap: 20px;
                margin-top: 40px;
            }
            .heading {
                display: flex;
                justify-content: space-between;
                align-items: center;
                grid-column: 1 / span 2;
            }
            .general-summary {
                height: 100%;
                color: var(--dark-gray-5);
            }
            ::slotted([slot="buttons"]) {
                display: flex;
                gap: 0 32px;
            }
            .input-container {
                padding: 31px 24px;
                border-radius: 12px;
                border: solid 2px #e6f0ff;
                overflow: auto;
            }
            h2 {
                margin-top: 0;
            }
            ::slotted([slot="inputs"]) {
                display: flex;
                flex-direction: column;
                //grid-template-rows: repeat(5, auto) 200px 200px;
                gap: 24px 0;
            }
            //::slotted([slot="users"]) {
            //    display: flex;
            //    flex-direction: row;
            //    //grid-template-rows: repeat(5, auto) 200px 200px;
            //    gap: 16px 0;
            //}
        `
    ];

    @property()
    editing_name: boolean = false;

    render() {
        return html`
            <div class="heading">
                <div>
                    <div class="general-summary">School Details</div>
                    <slot name="back"></slot>
                </div>
                <div class="heading-buttons">
                    <slot name="buttons"></slot>
                </div>
            </div>
            ${this.editing_name
                ? html`
                    <div class="input-container">
                        <slot name="internal"></slot>
                    </div>
                `
                : html`
                    <div class="input-container">
                        <h2>School</h2>
                        <slot name="inputs"></slot>
                    </div>
                    <div class="input-container">
                        <h2>Account</h2>
                        <slot name="account"></slot>
                    </div>
                    <div class="input-container">
                        <h2>Members</h2>
                        <slot name="users"></slot>
                    </div>
                `}
            <slot name="loader"></slot>
        `;
    }
}
