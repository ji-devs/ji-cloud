import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from "lit-html";
import "@elements/core/images/ui";

export type TitleKind = ""
    | 'background-image'
    | 'color'
    | 'overlay'
    | 'text'
    | 'image'
    | 'audio';

const STR_LABEL_LOOKUP: any = {
    ['background-image']: 'Image',
    ['color']: 'Color',
    ['overlay']: 'Overlay',
    ['text']: 'Text',
    ['image']: 'Image',
    ['audio']: 'Audio',
};

@customElement('menu-tab-title')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: flex;
                font-family: Poppins;
                font-size: 16px;
                font-weight: 500;
            }

            img-ui {
                max-width: 24px;
                max-height: 24px;
                margin-right: 8px;
                display: flex;
            }
        `];
    }

    @property()
    kind: TitleKind = "";

    @property()
    customLabel: string = "";

    render() {
        const { kind, customLabel } = this;

        const label = customLabel !== "" ? customLabel
            : STR_LABEL_LOOKUP[kind];

        return html`
            ${kind === "" ? nothing : html`
                <img-ui path="module/_common/sidebar/tab-${kind}-icon.svg"></img-ui>
            `}
            <div>${label}</div>
        `;
    }
}
