import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/images/ui";


@customElement('image-select-style-option')
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                display: flex;
                justify-content: space-between;
                cursor: pointer;
                padding: 2px 14px;
                align-items: center;
                line-height: 28px;
            }
            :host(:hover) {
                background-color: var(--light-blue-2);
            }
            .text {
                font-size: 16px;
                color: var(--dark-gray-6);
            }
        `];
    }

    @property({type: Boolean})
    selected: boolean = false;

    @property({type: String})
    label: string = "";

    render() {
        return html`
            <span class="text">${this.label}</span>
            ${ this.selected ? html`<img-ui path="module/_common/widgets/sidebar/image-select/filter-option-check.svg"></img-ui>` : '' }
        `;
    }
}
