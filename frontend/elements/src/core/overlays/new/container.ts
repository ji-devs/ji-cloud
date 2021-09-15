import { LitElement, html, css, customElement, property, query } from 'lit-element';
import { queryPierceShadow} from '@utils/dom';
@customElement("overlay-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    position: absolute;
                    top: 0;
                    left: 0;
                }
            `
        ];
    }

    firstUpdated() {
        const parentElement = queryPierceShadow(document, "#overlay");
        if(!parentElement) {
            console.warn("couldn't find #overlay!");
            return;
        }

        parentElement.appendChild(this);
    }

    render() {
        return html`
                <slot></slot>
        `;
    }
}
