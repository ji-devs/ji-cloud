import { LitElement, html, css, customElement, property } from 'lit-element';
import '@elements/core/images/ui';

export type Kind = '_somthing_' | 'matching' | 'flashcards' | 'print' | 'continue'; // TODO: fix _somthing_

const STR_LABEL_LOOKUP: {
    [key in Kind]: string
} = {
    ['_somthing_']: '_somthing_',
    ['matching']: 'Matching',
    ['flashcards']: 'Flashcards',
    ['print']: 'Print the cards',
    ['continue']: 'Continue',
};

@customElement('post-preview-action')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: inline-grid;
                cursor: pointer;
                row-gap: 6px;
            }
            .circle {
                height: 116px;
                width: 116px;
                border-radius: 50%;
                transition: background-color .3s;
                display: grid;
                place-content: center;
            }
            :host(:hover) .circle {
                background-color: var(--light-orange-3);
            }
            .label {
                text-align: center;
                transition: color .3s;
                line-height: 1.5;
            }
            :host(:hover) .label {
                color: var(--main-blue);
            }
        `];
    }

    @property()
    kind: Kind = "_somthing_";

    render() {
        return html`
            <div class="circle">
                <img-ui path="module/_common/post-preview/${this.kind}.svg"></img-ui>
            </div>
            <span class="label">${STR_LABEL_LOOKUP[this.kind]}</span>
        `;
    }
}
