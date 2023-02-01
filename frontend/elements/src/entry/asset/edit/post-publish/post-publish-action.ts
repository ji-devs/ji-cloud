import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
// import {ModuleKind, STR_MODULE_DISPLAY_NAME} from "@elements/module/_common/types";

export type Kind = "share" | "new" | "play" | "view-others";

@customElement("post-publish-action")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                    cursor: pointer;
                    row-gap: 6px;
                    justify-items: center;
                }
                .circle {
                    height: 116px;
                    width: 116px;
                    border-radius: 50%;
                    transition: background-color 0.3s;
                    display: grid;
                    place-content: center;
                }
                :host(:hover) .circle {
                    background-color: var(--light-orange-3);
                }
                .label {
                    transition: color 0.3s;
                    line-height: 1.5;
                    font-weight: 600;
                    color: #000000;
                }
                :host(:hover) .label {
                    color: var(--main-blue);
                }
            `,
        ];
    }

    @property()
    kind: Kind = "share";

    @property()
    assetDisplayName: string = "";

    private labelLookup() : string {
        switch (this.kind) {
            case "share":
                return `Share ${this.assetDisplayName}`;
            case "new":
                return `Create a new ${this.assetDisplayName}`;
            case "play":
                return `Play ${this.assetDisplayName}`;
            case "view-others":
                return `View my ${this.assetDisplayName}s`;
        }
    }

    render() {
        return html`
            <div class="circle">
                <img-ui
                    path="jig/edit/post-publish/action-${this.kind}.svg"
                ></img-ui>
            </div>
            <span class="label">${this.labelLookup()}</span>
        `;
    }
}
