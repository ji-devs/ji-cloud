import { LitElement, html, css, customElement, property, PropertyValues } from "lit-element";
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
    | "place"
    | "correct"
    | "incorrect"
    | "specific";

/*
font awesome icons
answer = message-dots
audio = volume
audio-file = file-audio
audio-record = microphone
background = ??
feedback = thumbs-up
fill-color = fill-drip
image = image
instructions = book-open
label = ??
overlay = ??
place = ??
play-settings = clapperboard-play
question = comment-question
select = ??
text = text
theme = ??
tooltip = ??
trace = circle-dashed
video ??
*/

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
    correct: "Correct",
    incorrect: "Incorrect",
    specific: "Specific",
};

const hasIcon = (kind: TabKind): boolean => {
    if (["correct", "incorrect", "specific"].includes(kind)) {
        return false;
    }

    return true;
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
                    font-size: 13px;
                    line-height: 1.2;
                }

                .highlight {
                    color: var(--main-blue);
                }
                .disabled {
                    color: #ccc;
                }

                img-ui {
                    max-width: 20px;
                    max-height: 20px;
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

    updated(propertyValues: PropertyValues) {
        if (propertyValues.has("kind")) {
            this.title = STR_LABEL_LOOKUP[this.kind];
        }
    }

    connectedCallback() {
        super.connectedCallback();

        this.addEventListener("mouseenter", this.onEnter);
        this.addEventListener("mouseleave", this.onLeave);
    }

    renderIcon(highlight: boolean) {
        const { kind } = this;

        const regularClass = classMap({ hidden: highlight });
        const activeClass = classMap({ hidden: !highlight });

        const iconUrl = `module/_common/edit/widgets/sidebar/menu-tabs/${kind}.svg`;
        const iconUrlActive = `module/_common/edit/widgets/sidebar/menu-tabs/${kind}-active.svg`;

        return this.kind === "" || !hasIcon(kind)
            ? nothing
            : html`
                <img-ui class=${regularClass} path="${iconUrl}"></img-ui>
                <img-ui
                    class=${activeClass}
                    path="${iconUrlActive}"
                ></img-ui>
            `
    }

    render() {
        const { kind, active, hover, disabled, small } = this;

        const highlight = active || hover;

        const label = STR_LABEL_LOOKUP[this.kind];

        const labelClass = classMap({
            highlight: highlight && !disabled,
            disabled,
        });


        return html`
            ${this.renderIcon(highlight)}
            ${small ? nothing : html`<div class=${labelClass}>${label}</div>`}
        `;
    }
}
