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
                    display: flex;
                    position: relative;
                    width: 40vw;
                    height: 40vw;
                    border: solid;
                    align-items: center; 
                    justify-content: center; 
                }

                ::slotted([slot=title]) {
                  display: grid;
                  position: relative;
                  justify-content: center;
                }

                ::slotted([slot=navigation]) {
                  display: grid;
                  position: relative;
                  justify-content: center;
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
