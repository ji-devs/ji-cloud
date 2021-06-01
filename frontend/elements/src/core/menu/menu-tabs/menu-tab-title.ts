import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from "lit-html";
import "@elements/core/images/ui";

export type TitleKind = ""
    | 'background-image'
    | 'color'
    | 'overlay'
    | 'text'
    | 'image'
    | 'audio'
    | 'play-settings'
    | 'instructions';

const STR_LABEL_LOOKUP: {
    [key in TitleKind]: string;
} = {
    ['']: '',
    ['background-image']: 'Image',
    ['color']: 'Color',
    ['overlay']: 'Overlay',
    ['text']: 'Text',
    ['image']: 'Image',
    ['audio']: 'Audio',
    ['play-settings']: 'Play Settings',
    ['instructions']: 'Instructions',
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

    @property({type: Boolean})
    active: boolean = false;

    render() {
        const label = STR_LABEL_LOOKUP[this.kind];
        const iconUrl = `module/_common/widgets/sidebar/tab-${this.kind}-icon${this.active ? "-active" : ""}.svg`;

        return html`
            ${this.kind === "" ? nothing : html`
                <img-ui path="${iconUrl}"></img-ui>
            `}
            <div>${label}</div>
        `;
    }
}
