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
                padding-top: 160px;
                box-sizing: border-box;
            }
            .jigzi-wrapper {
                grid-column: 1;
                grid-row: 1;
                display: grid;
                align-items: end;
                width: 1200px;
                max-width: 100vw;
            }
            main {
                display: grid;
                row-gap: 80px;
                align-self: start;
                justify-self: center;
                grid-column: 1;
                grid-row: 1;
            }
            h1 {
                margin: 0;
                font-size: 32px;
                font-weight: 900;
                color: var(--dark-blue-4);
            }
        `];
    }

    render() {
        return html`
            <div class="jigzi-wrapper">
                <slot name="jigzi"></slot>
            </div>
            <main>
                <h1>${STR_TYPE_THE_CODE}</h1>
                <slot name="input"></slot>
            </main>
        `;
    }
}
