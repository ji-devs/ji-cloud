import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

@customElement('module-sidebar-body')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: block;
                width: 492px;
                margin: 0 auto;
                min-height: 100%;
            }
        `];
    }

    render() {
        return html`<slot></slot>`
    }
}
