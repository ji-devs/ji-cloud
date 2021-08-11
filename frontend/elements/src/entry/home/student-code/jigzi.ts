import { LitElement, html, css, customElement, property } from 'lit-element';

export type Mode = "default" | "try-again" | "help";

@customElement('home-student-code-jigzi')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                overflow: hidden;
            }
            .button-wrapper-shadow {
                filter: drop-shadow(rgba(50, 50, 0, 0.5) 0px 2px 6px);
                /* should take up any space since it's translated away */
                width: 0;
            }
            ::slotted(button) {
                color: white;
                background-color: var(--dark-red-1);
                border: 0;
                padding: 0;
                cursor: pointer;
                height: 250px;
                width: 500px;
                font-size: 62px;
            }
            ::slotted(button) {
                display: none;
            }
            :host([mode=try-again]) ::slotted([slot=try-again]),
            :host([mode=help]) ::slotted([slot=help]) {
                display: inline-block;
            }


            :host([mode=try-again]) .button-wrapper {
                clip-path: path("M297.942 165.832L72.83 138.192l-54.37 21.354 14.162-26.291-10.607-1.3a24 24 0 0 1-20.9-26.746l10.24-83.377A24 24 0 0 1 38.1.936l275.93 33.88a24 24 0 0 1 20.9 26.746l-10.237 83.374a23.979 23.979 0 0 1-26.746 20.9z");
                transform: translate(300px, 350px) rotate(-14deg);
            }
            :host([mode=try-again]) ::slotted(button) {
                transform: rotate(7deg) translate(-65px, -36px);
                margin-left: -20px;
            }

            :host([mode=help]) .button-wrapper {
                clip-path: path("M348.5 192.264L97.712 161.471l-78.264 30.735 20.386-37.841L15 151.316a32 32 0 0 1-27.861-35.662l8.287-67.493A32 32 0 0 1 31.09 20.3l333.5 40.948a32 32 0 0 1 27.861 35.661L384.16 164.4a31.971 31.971 0 0 1-35.66 27.864z");
                transform: translate(300px, 350px);
            }
            :host([mode=help]) ::slotted(button) {
                transform: rotate(7deg) translate(-40px, -15px);
                margin-left: -20px;
            }
        `];
    }

    @property({ reflect: true })
    mode: Mode = "default";

    render() {
        return html`
            <div class="button-wrapper-shadow">
                <div class="button-wrapper">
                    <slot name="help"></slot>
                    <slot name="try-again"></slot>
                </div>
            </div>
            <img-ui path="entry/home/student-code/jigzi-${this.mode}.svg"></img-ui>
        `;
    }
}
