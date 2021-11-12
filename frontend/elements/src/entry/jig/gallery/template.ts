import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
import { styles } from "./styles";

export type Kind = "vocabulary" | "parsha";

interface KindInfo {
    label: string;
    ages: string;
}

const STR_INFO_LOOKUP: {
    [key in Kind]: KindInfo;
} = {
    ["vocabulary"]: {
        label: "Teach New Vocabulary",
        ages: "5-8",
    },
    ["parsha"]: {
        label: "Teach New Parsha",
        ages: "3-5",
    },
};

@customElement("jig-gallery-template")
export class _ extends LitElement {
    static get styles() {
        return [
            styles,
            css`
                :host {
                    display: inline-grid;
                    box-shadow: 0 3px 10px 0 rgba(0, 0, 0, 0.16);
                    border-radius: 16px;
                    cursor: pointer;
                    background-color: #fff;
                    overflow: hidden;
                    height: 176px;
                    width: 232px;
                    box-sizing: border-box;
                    grid-template-rows: auto min-content;
                }
                :host(:hover),
                :host(:focus) {
                    background-color: var(--light-blue-2);
                }
                .top-section {
                    justify-self: center;
                    padding: 16px;
                    display: grid;
                    grid-template-rows: auto min-content;
                    justify-items: center;
                    justify-content: space-between;
                }
                .label {
                    color: #4f4f4f;
                    font-weight: 600;
                }

                .ages {
                    background-color: var(--light-blue-3);
                    height: 40px;
                    color: var(--dark-gray-5);
                    justify-self: stretch;
                    justify-content: center;
                }
            `,
        ];
    }

    @property()
    kind: Kind = "vocabulary";

    render() {
        const info = STR_INFO_LOOKUP[this.kind];

        return html`
            <div class="top-section">
                <img-ui
                    path="entry/jig/gallery/template-${this.kind}.svg"
                ></img-ui>
                <span class="label">${info.label}</span>
            </div>
            <span class="ages">
                <img-ui path="entry/jig/gallery/age-icon.svg"></img-ui>
                ${info.ages}
            </span>
        `;
    }
}
