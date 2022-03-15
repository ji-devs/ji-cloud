import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
import {
    ModuleKind,
    STR_MODULE_DISPLAY_NAME,
} from "@elements/module/_common/types";

export type Kind = ModuleKind | "print" | "continue" | "publish";

const STR_LABEL_LOOKUP: { [key in Kind]: string } = {
    ...STR_MODULE_DISPLAY_NAME,
    print: "Print the cards",
    continue: "Add new activity",
    publish: "Publish JIG",
};

@customElement("post-preview-action")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                    cursor: pointer;
                    row-gap: 6px;
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
                    text-align: center;
                    transition: color 0.3s;
                    line-height: 1.5;
                    font-weight: 600;
                    width: 116px;
                }
                :host(:hover) .label {
                    color: var(--main-blue);
                }
            `,
        ];
    }

    @property()
    kind: Kind = "card-quiz";

    render() {
        const { kind } = this;

        const isModule = kind !== "continue" && kind !== "print" && kind !== "publish";

        const path = isModule
            ? `module/_common/edit/post-preview/module/${kind}.svg`
            : `module/_common/edit/post-preview/${this.kind}${
                  this.kind === "continue" || this.kind === "publish" ? ".png" : ".svg"
              }`;

        return html`
            <div class="circle">
                <img-ui path="${path}"></img-ui>
            </div>
            <span class="label">${STR_LABEL_LOOKUP[this.kind]}</span>
        `;
    }
}
