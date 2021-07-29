import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/images/ui";
import "@elements/core/progress-bar/progress-bar";

const STR_UPLOADING = "Uploading...";

@customElement("uploading-indicator")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    align-items: center;
                    column-gap: 14px;
                    height: 52px;
                }
                .icon-uploading {
                    width: 24px;
                }
                .text-uploading {
                    color: var(--main-blue);
                    font-size: 14px;
                }
                progress-bar {
                    width: 56px;
                    height: 8px;
                }
                .cancel {
                    height: 12px;
                    width: 12px;
                }
                ::slotted([slot=cancel]) {
                    height: 100%;
                    width: 100%;
                }
            `
        ];
    }

    @property({type: Number})
    progress: number = 1;

    render() {
        return html`
            <img-ui class="icon-uploading" path="core/uploading/uploading-icon.svg"></img-ui>
            <span class="text-uploading">${STR_UPLOADING}</span>
            <progress-bar color="green" progress="${this.progress}"></progress-bar>
            <div class="cancel">
                <slot name="cancel"></slot>
            </div>
        `;
    }
}
