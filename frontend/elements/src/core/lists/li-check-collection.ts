import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('li-check-collection')
export class _ extends LitElement {
    static get styles() {
        return [css`
            li {
                margin-bottom: 4px;
                list-style-type:  none;
                display: flex;
                justify-content: space-between;
                align-items:  center;
            }
            li:hover, :host([open]) li {
                background-color: #e7f0fe;
            }
            p {
                padding-left: 20px;
                padding-right: 20px;
                margin: 0;
            }
            img-ui {
                display: block;
                padding-right: 20px;
            }
        `];
    }

    @property({type: Boolean, reflect: true})
    open: boolean = false;

    render() {
        return html`
            <li>
                <p><slot></slot></p>
                <img-ui path="core/lists/angle-right.svg"></img-ui>
            </li>
        `;
    }
}
