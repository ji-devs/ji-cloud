import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import { ThemeId } from "@elements/_themes/themes";
import {
    cardBackFullPath,
    Mode,
    Side,
    StyleKind,
    getContentStyleConfig,
} from "@elements/module/_groups/cards/helpers";
import { styleMap } from "lit-html/directives/style-map";

// this is kept separate from "play-card" because:
// 1. Values are in px not rem
// 2. It's often displayed in dual-pairs and may need slight styling changes for that
// 3. There is only one size to deal with

@customElement("main-card")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    --card-size: 160px;
                    --border-size: 1px;
                    --img-padding: 10px;

                    --theme-color: --theme-blank-cards-color;
                    --theme-border-color: --theme-blank-cards-border-color;
                    --theme-border-color-light: --theme-blank-cards-border-color-light-hsl;
                    --theme-background-color: --theme-blank-cards-fill-color;
                    --theme-font-family: --theme-blank-cards-font-family;

                    /* Required for child elements which make use of these */
                    --color: var(--theme-color);
                    --font-family: var(--theme-font-family);
                }
                section {
                    transition: transform 0.8s;
                    transform-style: preserve-3d;
                }

                :host([dragOver]) section.editing .front {
                    border-style: dashed;
                    border-width: 3px;
                    border-color: rgb(var(--theme-border-color));
                    margin: 0;
                }

                :host([selected]) section.editing .front {
                    border-width: 3px;
                    border-color: rgb(var(--theme-border-color));
                    margin: 0;
                }

                .front {
                    border-style: solid;
                    border-radius: 16px;
                    border-width: var(--border-size);

                    border-color: hsl(var(--theme-border-color-light));
                    background-color: rgb(var(--theme-background-color));
                    margin: 2px;
                }

                section.editing .front:hover {
                    border-color: rgb(var(--theme-border-color));
                }

                .front,
                .back,
                .back > img-ui {
                    box-sizing: border-box;
                    width: 100%;
                    height: 100%;
                }

                section {
                    width: var(--card-size);
                    height: var(--card-size);
                }

                :host([inverted]) section {
                    transform: rotateY(180deg);
                }

                :host([inverted]) section.flippable:hover {
                    transform: rotateY(0);
                }
                section.flippable:hover {
                    transform: rotateY(180deg);
                }

                .front,
                .back {
                    display: grid;
                    position: absolute;
                    -webkit-backface-visibility: hidden; /* Safari */
                    backface-visibility: hidden;
                }

                .back {
                    transform: rotateY(180deg);
                }
                .back > img-ui {
                    object-fit: cover;
                }
                ::slotted(*) {
                    display: grid;
                    place-content: center;
                    --img-size: calc(
                        var(--card-size) -
                            ((var(--border-size) * 2) + var(--img-padding))
                    );
                    padding: calc(var(--img-padding) / 2);
                    width: var(--img-size);
                    height: var(--img-size);
                    object-fit: contain;
                }
                slot[name="menu"]::slotted(*) {
                    position: absolute;
                    top: -20px;
                    left: 137px;
                    display: inline-block;
                    width: 32px;
                    height: 32px;
                }

                slot[name="audio"]::slotted(*) {
                    position: absolute;
                    top: -20px;
                    left: 100px;
                    display: inline-block;
                    width: 32px;
                    height: 32px;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    dragOver: boolean = false;

    @property({ type: Boolean, reflect: true })
    selected: boolean = false;

    @property({ type: Boolean })
    flippable: boolean = false;

    @property({ type: Boolean, reflect: true })
    inverted: boolean = false;

    @property()
    theme: ThemeId = "blank";

    @property({ type: Boolean })
    editing: boolean = false;

    @property({ reflect: true })
    side: Side = "left";

    @property()
    mode: Mode = "duplicate";

    // style mode - see helpers definition
    @property()
    styleKind: StyleKind = "theme";

    updated() {
        const { theme, mode, side, styleKind } = this;
        const styleConfig = getContentStyleConfig(theme, mode, side);
        this.style.setProperty("--theme-color", styleConfig.color);
        this.style.setProperty("--theme-border-color", styleConfig.borderColor);
        this.style.setProperty("--theme-border-color-light", styleConfig.borderColorLight);
        this.style.setProperty("--theme-background-color", styleConfig.backgroundColor);
        this.style.setProperty("--theme-font-family", styleConfig.fontFamily);
    }

    render() {
        const { flippable, theme, editing } = this;

        return html`
            <section class="${classMap({ flippable, editing })}">
                <div class="front"><slot></slot></div>
                <div class="back">
                    <img-ui path="${cardBackFullPath(theme)}"></img-ui>
                </div>
                <div class="audio"><slot name="audio"></slot></div>
                <div class="menu"><slot name="menu"></slot></div>
            </section>
        `;
    }
}
