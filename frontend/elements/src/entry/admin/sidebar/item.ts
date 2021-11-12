import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

export type ID =
    | "locale"
    | "image-add"
    | "image-search"
    | "jigs"
    | "category"
    | "image-tags";

const STR_LABEL_LOOKUP: { [key in ID]: string } = {
    "image-add": "Add image",
    "image-tags": "Image tags",
    "image-search": "Edit images",
    jigs: "Label JIGs",
    category: "Edit categories",
    locale: "Localization",
};

@customElement("admin-sidebar-item")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host([locked]) {
                    pointer-events: none;
                }

                section {
                    max-width: 259px;
                    height: 56px;
                    cursor: pointer;
                    display: flex;
                    align-items: center;
                    border-left: solid 8px #83aef7;
                    justify-content: space-between;
                    padding-right: 20px;
                }

                :host([selected]) > section,
                section:hover {
                    background-color: #6698ed;
                    border-left: solid 8px #2b54b8;
                }

                p {
                    font-size: 18px;
                    font-weight: 500;
                    margin-left: 40px;
                }
            `,
        ];
    }

    @property()
    id: ID = "image-add";

    @property({ type: Boolean, reflect: true })
    locked: boolean = false;

    @property({ type: Boolean, reflect: true })
    selected: boolean = false;

    render() {
        const { id, locked } = this;

        const label = STR_LABEL_LOOKUP[id];

        return html`
            <section>
                <p>${label}</p>
                ${locked
                    ? html`<img-ui
                          path="entry/admin/sidebar/lock.svg"
                      ></img-ui>`
                    : nothing}
            </section>
        `;
    }
}
