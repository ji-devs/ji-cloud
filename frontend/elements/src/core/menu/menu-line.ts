import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/images/ui";

type IconKind =
    | ""
    | "copy"
    | "delete"
    | "duplicate"
    | "edit"
    | "move-down"
    | "move-up"
    | "paste"
    | "print"
    | "reuse"
    | "set-jig-theme"
    | "text"
    | "record-sound"
    | "image"
;

const STR_LABEL_LOOKUP: any = {
    copy: "Copy",
    paste: "Paste",
    delete: "Delete",
    duplicate: "Duplicate",
    edit: "Edit",
    ["move-down"]: "Move Down",
    ["move-up"]: "Move Up",
    ["print"]: "Print",
    ["reuse"]: "Reuse",
    //all stickers
    ["move-to-front"]: "Bring to front",
    ["move-forward"]: "Bring forwards",
    ["move-backward"]: "Send backwards",
    ["move-to-back"]: "Send to back",
    ["flip-horizontal"]: "Flip right-left",
    ["flip-vertical"]: "Flip up-down",
    //bg only
    ["change-background-color"]: "Change background color",
    ["change-background-image"]: "Change background image",
    ["remove-background-image"]: "Remove background image",
    ["remove-overlay"]: "Remove overlay",
    //image only
    ["crop"]: "Crop (coming soon!)",
    ["remove-white"]: "Remove white",
    ["make-background"]: "Make background",
    ["play"]: "Play",
    ["record-sound"]: "Record sound",
    ["upload-sound"]: "Upload sound",
    //module publish
    ["use-content-as"]: "Use content as",
    //Theme selector
    ["set-jig-theme"]: "Set JIG theme",
    ["text"]: "Edit text",
};

@customElement("menu-line")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                section {
                    display: flex;
                    cursor: pointer;
                }

                .img {
                    width: 24px;
                    height: 24px;
                    margin-right: 13px;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }

                .label {
                    white-space: nowrap;
                }

                :host([active]),
                .hover {
                    color: #5590fc;
                }

                .delete {
                    color: #f84f57;
                }
            `,
        ];
    }

    onEnter() {
        this.hover = true;
    }

    onLeave() {
        this.hover = false;
    }

    @property({ type: Boolean })
    hover: boolean = false;

    @property()
    icon: IconKind = "";

    @property()
    customLabel: string = "";

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    firstUpdated() {
        this.tabIndex = 0;
    }

    render() {
        const { icon, customLabel, active } = this;
        let { hover } = this;

        if (icon === "delete") {
            hover = false;
        }

        const label = customLabel !== "" ? customLabel : STR_LABEL_LOOKUP[icon];

        const labelClasses = classMap({
            label,
            hover,
            delete: icon === "delete",
        });

        const filename = hover || active ? `${icon}-hover` : icon;

        return html`
            <section
                @mouseenter="${this.onEnter}"
                @mouseleave="${this.onLeave}"
            >
                <div class="img">
                    ${icon === ""
                        ? nothing
                        : html`
                              <img-ui
                                  path="core/menus/${filename}.svg"
                              ></img-ui>
                          `}
                </div>
                <div class="label ${labelClasses}">${label}</div>
            </section>
        `;
    }
}
