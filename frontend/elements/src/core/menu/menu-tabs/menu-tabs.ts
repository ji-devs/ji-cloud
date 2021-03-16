import { LitElement, html, css, customElement } from 'lit-element';


@customElement('menu-tabs')
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                display: grid;
            }
            .body {
                /* hard coded color?! Couldn't find color in zeplin */
                background-color: #e9eff8;
            }
        `];
    }

    render() {
        return html`
            <div>
                <slot name="tabs"></slot>
            </div>
            <div class="body">
                <slot name="body"></slot>
            </div>
        `;
    }
}
