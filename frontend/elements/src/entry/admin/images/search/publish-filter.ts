import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";

export type Mode = "all" | "published" | "saved";

const STR_ALL = "Show all";
const STR_PUBLISHED = "Show published";
const STR_SAVED = "Show saved";

@customElement("image-search-publish-filter")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    margin: 10px 10px;
                }
                :host([mode="published"])::before {
                    background-color: #6eca90;
                    content: "";
                    height: 16px;
                    width: 16px;
                    border-radius: 50%;
                    display: inline-block;
                    margin-right: 8px;
                }
                :host([mode="saved"])::before {
                    background-color: #e36486;
                    content: "";
                    height: 16px;
                    width: 16px;
                    border-radius: 50%;
                    display: inline-block;
                    margin-right: 8px;
                }
            `,
        ];
    }

    @property({ reflect: true })
    mode: Mode = "published";

    render() {
        const { mode } = this;

        const label =
            mode === "published"
                ? STR_PUBLISHED
                : mode === "saved"
                ? STR_SAVED
                : STR_ALL;

        return html`${label}`;
    }
}
