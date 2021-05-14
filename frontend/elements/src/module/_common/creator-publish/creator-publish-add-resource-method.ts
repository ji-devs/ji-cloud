import { LitElement, html, css, customElement, property, unsafeCSS, internalProperty } from 'lit-element';
import "@elements/core/images/ui";

export type Kind = 'upload' | 'link';

const STR_LABEL_LOOKUP: {
    [key in Kind]: string
} = {
    ['upload']: "Upload file",
    ['link']: "Add link",
};


@customElement('creator-publish-add-resource-method')
export class _ extends LitElement {
    static get styles() {
        return [css`
            .add-button {
                cursor: pointer;
                display: flex;
                align-items: center;
                column-gap: 8px;
            }
            .label {
                font-size: 13px;
                font-weight: 500;
                color: var(--dark-gray-6);
            }
            :host(:hover) .label {
                color: var(--main-blue);
            }
      `];
    }

    @property()
    kind: Kind = 'upload';

    render() {
        return html`
            <div class="add-button">
                <img-ui path=""></img-ui>
                <span class="label">${STR_LABEL_LOOKUP[this.kind]}</span>
            </div>
        `;
    }
}
