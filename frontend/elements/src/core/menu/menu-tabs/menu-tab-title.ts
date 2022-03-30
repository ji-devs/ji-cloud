import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/images/ui";

export type TabKind =
    | ""
    | "answer"
    | "audio"
    | "background-image"
    | "fill-color"
    | "feedback"
    | "image"
    | "instructions"
    | "label"
    | "overlay"
    | "play-settings"
    | "question"
    | "select"
    | "text"
    | "theme"
    | "tooltip"
    | "video"
    | "trace"
    | "place";

const STR_ICON_OVERRIDE: Partial<{
    [key in TabKind]: TabKind;
}> = {
    // "add-text": "tooltip",
    // "background-image-full": "background-image"
    // "background-color": "color"
};

const STR_LABEL_LOOKUP: {
    [key in TabKind]: string;
} = {
    "": "",
    answer: "Answer",
    audio: "Audio",
    "background-image": "Background",
    "fill-color": "Fill Color",
    feedback: "Feedback",
    image: "Image",
    instructions: "Instructions",
    label: "Label",
    overlay: "Overlay",
    "play-settings": "Play Settings",
    question: "Question",
    select: "Select",
    text: "Text",
    theme: "Theme",
    tooltip: "Tooltip", //Not in zeplin
    video: "Video", //Not in Zeplin
    trace: "Trace",
    place: "Place",
};

const getIcon = (kind: TabKind): TabKind => {
    const override = STR_ICON_OVERRIDE[kind];

    if (override != null) {
        return override;
    } else {
        return kind;
    }
};

@customElement("menu-tab-title")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    font-family: Poppins;
                    font-weight: 500;
                    align-items: center;
                    font-size: 14px;
                    line-height: 1.2;
                }
                @media (min-width: 1920px) {
                    :host {
                        font-size: 16px;
                    }
                }

                .highlight {
                    color: var(--main-blue);
                }
                .disabled {
                    color: #ccc;
                }

                img-ui {
                    max-width: 24px;
                    max-height: 24px;
                    margin-right: 8px;
                    display: flex;
                }

                .hidden {
                    display: none;
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
    kind: TabKind = "";

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    @property({ type: Boolean, reflect: true })
    disabled: boolean = false;

    @property({ type: Boolean })
    small: boolean = false;

    connectedCallback() {
        super.connectedCallback();

        this.addEventListener("mouseenter", this.onEnter);
        this.addEventListener("mouseleave", this.onLeave);
    }

    render() {
        const { kind, active, hover, disabled, small } = this;

        const highlight = active || hover;

        const label = STR_LABEL_LOOKUP[this.kind];
        const iconUrl = `module/_common/edit/widgets/sidebar/menu-tabs/${getIcon(
            kind
        )}.svg`;
        const iconUrlActive = `module/_common/edit/widgets/sidebar/menu-tabs/${getIcon(
            kind
        )}-active.svg`;

        const regularClass = classMap({ hidden: highlight });
        const activeClass = classMap({ hidden: !highlight });

        const labelClass = classMap({
            highlight: highlight && !disabled,
            disabled,
        });
        return html`
            ${this.kind === ""
                ? nothing
                : html`
                      <img-ui class=${regularClass} path="${iconUrl}"></img-ui>
                      <img-ui
                          class=${activeClass}
                          path="${iconUrlActive}"
                      ></img-ui>
                  `}
            ${small ? nothing : html`<div class=${labelClass}>${label}</div>`}
        `;
    }
}
