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
    | "jig-info"
    | "jig-play"
    | "view"
    | "share"
;

const STR_LABEL_LOOKUP: any = {
    copy: "Copy",
    paste: "Paste",
    delete: "Delete",
    duplicate: "Duplicate",
    edit: "Edit",
    animations: "Animations",
    ["jig-info"]: "JIG info",
    ["jig-play"]: "Play JIG",
    ["move-down"]: "Move Down",
    ["move-up"]: "Move Up",
    ["print"]: "Print",
    ["reuse"]: "Reuse",
    //all stickers
    ["move-to-front"]: "Bring to front",
    ["move-forward"]: "Bring forward",
    ["move-backward"]: "Send backward",
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
    ["view"]: "View",
    ["record-sound"]: "Record sound",
    ["upload-sound"]: "Upload sound",
    //module publish
    ["use-content-as"]: "Use content as",
    //Theme selector
    ["set-jig-theme"]: "Set JIG theme",
    ["text"]: "Edit text",
    ["share"]: "Share",
};

// possible fa icons
// copy   <i class="fa-regular fa-copy"></i>
// delete   <i class="fa-regular fa-trash-can"></i>
// duplicate   <i class="fa-regular fa-copy"></i>
// edit   <i class="fa-regular fa-pen"></i>
// move-down   <i class="fa-regular fa-down-to-line"></i>
// move-up   <i class="fa-regular fa-up-to-line"></i>
// paste   <i class="fa-regular fa-paste"></i>
// print   <i class="fa-regular fa-print"></i>
// reuse   <i class="fa-regular fa-arrows-rotate"></i>
// set-jig-theme   ???
// text   <i class="fa-regular fa-text"></i>
// record-sound   <i class="fa-regular fa-microphone"></i>
// image   <i class="fa-regular fa-image"></i>
// info   <i class="fa-regular fa-file-lines"></i>
// play   <i class="fa-regular fa-clapperboard-play"></i>

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
