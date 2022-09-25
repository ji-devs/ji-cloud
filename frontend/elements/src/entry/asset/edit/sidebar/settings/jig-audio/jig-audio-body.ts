import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";

export type Kind = "background" | "feedback";

const STR_HEADER: {
    [key in Kind]: string;
} = {
    ["background"]: "Add Background Music",
    ["feedback"]: "Feedback Effects",
};

@customElement("jig-audio-body")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                popup-body {
                    width: 424px;
                }
                .search-and-custom {
                    display: grid;
                    grid-template-columns: 1fr auto;
                    column-gap: 30px;
                    align-items: center;
                    padding: 0 32px;
                }
                .correct-mistake {
                    padding-bottom: 16px;
                    margin: 0 32px 24px 32px;
                    border-bottom: solid 1px #d5e4ff;
                    display: flex;
                    column-gap: 24px;
                }
                :host([kind="background"]) .correct-mistake {
                    display: none;
                }
                .lines {
                    grid-column: 1 / -1;
                    max-height: 340px;
                    overflow: auto;
                    padding-bottom: 16px;
                }
            `,
        ];
    }

    @property({ reflect: true })
    kind: Kind = "background";

    render() {
        return html`
            <popup-body>
                <slot name="back" slot="back"></slot>
                <slot name="close" slot="close"></slot>
                <h2 slot="heading">${STR_HEADER[this.kind]}</h2>
                <div class="body" slot="body">
                    <div class="correct-mistake">
                        <slot name="correct-mistake"></slot>
                    </div>
                    <div class="search-and-custom">
                        <slot name="search"></slot>
                        <slot name="add-custom"></slot>
                    </div>
                    <div class="lines">
                        <slot name="lines"></slot>
                    </div>
                </div>
            </popup-body>
        `;
    }
}
