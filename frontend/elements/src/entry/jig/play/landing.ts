import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("jig-play-landing")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    height: 100vh;
                    width: 100vw;
                }
                main {
                    display: grid;
                    height: 100%;
                    width: 100%;
                }
                ::slotted([slot=iframe]), .overlay {
                    grid-column: 1;
                    grid-row: 1;
                    height: 100%;
                    width: 100%;
                    border: 0;
                }
                .overlay {
                    z-index: 1;
                    pointer-events: none;
                    display: grid;
                    grid-template-columns: 0px 1fr 170px;
                    grid-template-rows: 1fr 100px;
                }
                .overlay ::slotted(*) {
                    pointer-events: all;
                }
                .sidebar {
                    grid-column: 1;
                    grid-row: 1 / -1;
                    z-index: 2;
                }
                .side-bar {
                    grid-column: 3;
                    grid-row: 1;
                    display: grid;
                    grid-gap: 16px;
                    justify-items: center;
                    align-content: start;
                    padding: 32px;
                }
                .side-bar .replay-background {
                    display: flex;
                    justify-content: space-between;
                    column-gap: 12px;
                }
                .bottom-bar {
                    grid-row: 2;
                    grid-column: 1 / -1;
                    display: grid;
                    grid-template-columns: auto 450px auto;
                    align-items: center;
                    justify-content: center;
                    grid-gap: 16px;
                }
            `,
        ];
    }

    render() {
        return html`
            <main>
                <slot name="iframe"></slot>
                <div class="overlay">
                    <div class="sidebar">
                        <slot name="sidebar"></slot>
                    </div>
                    <div class="side-bar">
                        <slot name="play-pause-button"></slot>
                        <div class="replay-background">
                            <slot name="replay-background"></slot>
                        </div>
                        <slot name="indicators"></slot>
                    </div>
                    <div class="bottom-bar">
                        <slot name="progress"></slot>
                    </div>
                </div>
            </main>
        `;
    }
}
