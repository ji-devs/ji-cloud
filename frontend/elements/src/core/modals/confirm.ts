import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { BaseButton } from "@elements/_styles/buttons";
import "@elements/core/buttons/icon";
import "@elements/core/buttons/rectangle";

const STR_DEFAULT_CANCEL_TEXT = "Cancel";
const STR_DEFAULT_CONFIRM_TEXT = "Confirm";
const STR_TITLE_WARNING = "Warning";

@customElement("modal-confirm")
export class _ extends BaseButton {
    static get styles() {
        return [
            css`
                .overlay {
                    position: fixed;
                    top: 0;
                    left: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    width: 100vw;
                    height: 100vh;
                    opacity: 0.8;
                    background-color: var(--light-blue-3);
                    z-index: 6;
                }
                .container {
                    position: fixed;
                    top: 0;
                    left: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    width: 100vw;
                    height: 100vh;
                    z-index: 7;
                }
                .section {
                    width: 419px;
                    min-height: 276px;
                    border-radius: 16px;
                    -webkit-backdrop-filter: blur(30px);
                    backdrop-filter: blur(30px);
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.16);
                    background-color: var(--white);
                    opacity: 1;
                    display: flex;
                    flex-direction: column;
                }

                .close {
                    align-self: flex-end;
                    margin: .5em .5em 0 0;
                }

                .contents {
                    display: flex;
                    flex-direction: column;
                    padding: 0 32px; 32px; 32px;
                    flex: 1;
                }
                .warning {
                    color: var(--dark-red-2);
                }

                .divider {
                  width: 354px;
                  height: 0;
                  border: solid 1px #d5e4ff;
                  margin-top: 16px;
                  margin-bottom: 24px;
                }
                .title {
                    font-size: 32px;
                    font-weight: bold;
                    font-stretch: normal;
                    font-style: normal;
                    line-height: 1.5;
                    letter-spacing: -0.32px;
                    text-align: left;
                }
                .content {
                  font-size: 16px;
                  font-weight: normal;
                  font-stretch: normal;
                  font-style: normal;
                  line-height: 1.5;
                  letter-spacing: normal;
                  text-align: left;
                  color: var(--dark-gray-6);
                  flex: 1;
                }
                .options {
                    margin: 2em 0 1em 0;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }
                `,
        ];
    }

    onAnyClick(evt: MouseEvent) {
        const path = evt.composedPath();
        // Makes sure that only clicking the overlay will trigger a cancel event.
        if (!path.includes(this.shadowRoot?.getElementById("section") as any)) {
            this.onCancel();
        }
    }

    onCancel() {
        this.dispatchEvent(
            new CustomEvent("custom-cancel", {})
        );
    }

    onConfirm() {
        this.dispatchEvent(
            new CustomEvent("custom-confirm", {})
        );
    }

    @property({ type: String })
    title!: string;

    @property({ type: String })
    content!: string;

    @property({ type: String })
    cancel_text: string = STR_DEFAULT_CANCEL_TEXT;

    @property({ type: String })
    confirm_text: string = STR_DEFAULT_CONFIRM_TEXT;

    @property({ type: Boolean })
    dangerous: boolean = false;

    buttonProps(isPrimary: Boolean) {
        let color = "blue";
        let kind = "filled";

        if (!isPrimary && this.dangerous) {
            color = "red";
            kind = "text";
        } else if (!isPrimary && !this.dangerous) {
            kind = "text";
        }

        return [color, kind];
    }

    renderConfirm(isPrimary: Boolean) {
        const [color, kind] = this.buttonProps(isPrimary);

        return html`
            <div @click=${this.onConfirm}>
                <button-rect color=${color} kind=${kind}>${this.confirm_text}</button-rect>
            </div>
        `;
    }

    renderCancel(isPrimary: Boolean) {
        const [color, kind] = this.buttonProps(isPrimary);

        return html`
            <div @click=${this.onCancel}>
                <button-rect color=${color} kind=${kind}>${this.cancel_text}</button-rect>
            </div>
        `;
    }

    renderActions() {
        if (this.dangerous) {
            return html`
                <div class="options">
                    ${this.renderConfirm(false)}
                    ${this.renderCancel(true)}
                </div>
            `;
        } else {
            return html`
                <div class="options">
                    ${this.renderCancel(false)}
                    ${this.renderConfirm(true)}
                </div>
            `;
        }
    }

    renderTitle() {
        const titleClasses = classMap({
            title: true,
            warning: this.dangerous,
        });


        return html`
            <div class="${titleClasses}">${this.title}</div>
        `;
    }

    renderContent() {
        return html`
            <div class="content">${this.content}</div>
        `;
    }

    render() {
        return html`
            <div class="overlay">
            </div>
            <div class="container" @click=${this.onAnyClick}>
                <div class="section">
                    <button-icon
                        size="small"
                        class="close"
                        icon="x"
                        @click=${this.onCancel}
                    ></button-icon>
                    <div class="contents">
                        ${this.renderTitle()}
                        <div class="divider"></div>
                        ${this.renderContent()}
                        ${this.renderActions()}
                    </div>
                </div>
            </div>
        `;
    }
}
