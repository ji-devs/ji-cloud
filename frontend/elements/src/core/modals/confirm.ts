import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import { nothing} from 'lit-html';
import {BaseButton} from "@elements/_styles/buttons";
import "@elements/core/buttons/icon";
import "@elements/core/buttons/rectangle";

export type Mode = "deleteModule";

const STR_TITLE_WARNING = "Warning";
const STR_BODY_DELETE_MODULE = "Are you sure you want to delete this activity?";
const STR_CONFIRM_DELETE_MODULE = "Delete activity";
const STR_CANCEL_DELETE_MODULE = "Don't delete";

@customElement("modal-confirm")
export class _ extends BaseButton {
    static get styles() {
        return [
            css`
                :host {
                    position: fixed;
                    top: 0;
                    left: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    width: 100vw;
                    height: 100vh;
                    opacity: 0.8;
                    background-color: var(--Light_Blue_3);
                }
                section {
                    width: 419px;
                    height: 276px;
                    border-radius: 16px;
                    -webkit-backdrop-filter: blur(30px);
                    backdrop-filter: blur(30px);
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.16);
                    background-color: var(--white);

                    display: flex;
                    flex-direction: column;
                }


                .close {
                    align-self: flex-end;
                }

                .contents {
                    display: flex;
                    flex-direction: column;
                    padding: 0 32px; 32px; 32px;
                }
                .warning {
                    color: var(--Dark_Red_2);
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
                .body {
                  font-size: 16px;
                  font-weight: normal;
                  font-stretch: normal;
                  font-style: normal;
                  line-height: 1.5;
                  letter-spacing: normal;
                  text-align: left;
                  color: var(--Dark_Gray_6);
                }
                .options {
                    margin-top: 40px;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }
                .confirm-warning {
                  font-size: 16px;
                  font-weight: 500;
                  font-stretch: normal;
                  font-style: normal;
                  line-height: 1.5;
                  letter-spacing: normal;
                  text-align: center;
                  color: var(--Dark_Red_2);
                  cursor: pointer;
                }
                `];
    }

    @property()
    mode: Mode = "deleteModule";

    render() {


        const {mode} = this;

        const title = STR_TITLE_WARNING;
        const body = STR_BODY_DELETE_MODULE;
        const confirm = html`<div class="confirm-warning">${STR_CONFIRM_DELETE_MODULE}</div>`
        const cancel = html`<button-rect color="blue">${STR_CANCEL_DELETE_MODULE}</button-rect>`

        const titleClasses = classMap({
            title: true,
            warning: mode === "deleteModule"
        });

        return html`
            <section>
                <button-icon class="close" icon="x"></button-icon>
                <div class="contents">
                    <div class="${titleClasses}">${title}</div>
                    <div class="divider"></div>
                    <div class="body">${body}</div>
                    <div class="options">
                        <div class="confirm">${confirm}</div>
                        <div class="cancel">${cancel}</div>
                    </div>
                </div>
            </section>
        `;
    }
}

