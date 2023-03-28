import { LitElement, html, css, customElement } from "lit-element";

@customElement("asset-edit-course-selection")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    padding: 24px;
                    display: grid;
                    overflow: auto;
                    max-height: 100dvh;
                    box-sizing: border-box;
                }
                h1 {
                    font-size: 24px;
                    font-weight: 900;
                    color: var(--dark-blue-4);
                    margin: 0;
                    text-align: center;
                }
                h3 {
                    font-size: 18px;
                    font-weight: 500;
                    color: var(--dark-blue-4);
                    max-width: 465px;
                    text-align: center;
                    margin: 0 auto 24px auto;
                }
                ::slotted([slot=search-bar]) {
                    margin: 0 auto 16px auto;
                    max-width: 660px;
                }
                ::slotted([slot=rated]) {
                    margin: auto;
                    max-width: 80%;
                    width: 550px;
                }
                ::slotted([slot=dragging]) {
                    position: fixed;
                    top: 0;
                    left: 0;
                    z-index: 1;
                    cursor: grabbing;
                    touch-action: none;
                    user-select: none;
                    pointer-events: none;
                }
            `,
        ];
    }

    render() {
        return html`
            <h1>New Course</h1>
            <h3>Search for JIGs or resources and drag them to your course journey</h3>
            <slot name="search-bar"></slot>
            <slot name="rated"></slot>
            <slot name="results"></slot>
            <slot name="dragging"></slot>
            <slot name="player"></slot>
        `;
    }
}
