import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import { ThemeId } from "@elements/_themes/themes";
import {
    cardBackFullPath,
    Mode,
    Side,
    StyleKind,
    getContentStyle,
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
                }
                section {
                    transition: transform 0.8s;
                    transform-style: preserve-3d;
                }

                :host([dragOver]) section.editing .front {
                    border-style: dashed;
                    border-radius: 16px;
                    border-width: 3px;
                    background-color: var(--light-blue-1);
                }

                .front {
                    border-style: solid;
                    border-radius: 16px;
                    border-width: var(--border-size);
                    background-color: white;
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

                ::slotted(*) {
                    --img-size: calc(
                        var(--card-size) -
                            ((var(--border-size) * 2) + var(--img-padding))
                    );
                    padding: calc(var(--img-padding) / 2);
                    width: var(--img-size);
                    height: var(--img-size);
                    object-fit: contain;
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
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    dragOver: boolean = false;

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

    render() {
        const { flippable, theme, editing, mode, side, styleKind } = this;

        const contentStyle = getContentStyle(styleKind, theme, mode, side);

        return html`
            <section class="${classMap({ flippable, editing })}">
                <div class="front" style=${contentStyle}><slot></slot></div>
                <div class="back">
                    <img-ui path="${cardBackFullPath(theme)}"></img-ui>
                </div>
            </section>
        `;
    }
}
