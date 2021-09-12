import { LitElement, html, css, customElement, queryAll } from 'lit-element';

const STR_TYPE_THE_CODE = "Type in the code your teacher gave you";

@customElement('kids-student-code')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: grid;
                justify-content: center;
                background-color: var(--green-3);
                height: 100vh;
                width: 100vw;
                box-sizing: border-box;
            }
            main {
                display: grid;
                row-gap: 80px;
                grid-column: 1;
                grid-row: 1;
                width: 100vw;
                align-content: start;
                z-index: 1;
            }
            header {
                background-color: white;
                height: 88px;
                display: grid;
                align-items: center;
                padding: 0 25px;
            }
            h1 {
                margin: 0;
                margin-top: 80px;
                font-size: 32px;
                font-weight: 900;
                color: var(--dark-blue-4);
                text-align: center;
            }
            ::slotted(kids-student-code-input) {
                justify-self: center;
            }
            .jigzi-wrapper {
                justify-self: center;
                grid-column: 1;
                grid-row: 1;
                display: grid;
                align-items: end;
                width: 1200px;
                max-width: 100vw;
            }
        `];
    }

    render() {
        return html`
            <main>
                <header>
                    <img-ui path="core/page-header/logo.svg"></img-ui>
                </header>
                <h1>${STR_TYPE_THE_CODE}</h1>
                <slot name="input"></slot>
            </main>
            <div class="jigzi-wrapper">
                <slot name="jigzi"></slot>
            </div>
        `;
    }
}
