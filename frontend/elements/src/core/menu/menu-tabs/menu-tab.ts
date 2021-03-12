import { LitElement, html, css, customElement, property} from 'lit-element';


@customElement('menu-tab')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: inline-grid;
                border-top-left-radius: 10px;
                border-top-right-radius: 10px;
                padding: 13px 11px;
                cursor: pointer
            }
            :host([active]) {
                /* hard coded color?! Couldn't find color in zeplin */
                background-color: #e9eff8;
                color: var(--Main_Blue);
            }
        `];
    }

    @property({type: Boolean, reflect: true})
    active: boolean = false;

    render() {
        return html`
            <slot></slot>
        `;
    }
}
