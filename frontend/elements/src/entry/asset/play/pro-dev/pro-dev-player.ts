import { LitElement, html, css, customElement, property, state } from "lit-element";

@customElement("pro-dev-player")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    height: 100vh;
                    justify-content: center;
                    align-items: center;
                }
              
                ::slotted([slot=player-window]) {
                    display: grid;
                    position: relative;
                }

                ::slotted([slot=title]) {
                  display: grid;
                  position: relative;
                }

                ::slotted([slot=navigation]) {
                  display: grid;
                  position: relative;
                }
            `,
        ];
    }

    render() {
        return html`
            <div>
                <slot name="player-window"></slot>
                <slot name="title"></slot>
                <slot name="navigation"></slot>
            </div>
        `;
    }
}
