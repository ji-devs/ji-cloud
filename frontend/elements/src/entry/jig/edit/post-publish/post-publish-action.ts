import { LitElement, html, css, customElement, property } from 'lit-element';
import '@elements/core/images/ui';
// import {ModuleKind, STR_MODULE_DISPLAY_NAME} from "@elements/module/_common/types";

export type Kind = 'share' | 'new-jig' | 'play-jig';

const STR_LABEL_LOOKUP:{[key in Kind]:string} = {
    'share': 'Share the JIG',
    'new-jig': 'Create a new JIG',
    'play-jig': 'Play the JIG',
};

const ext: {
    [key in Kind]: string
} = {
    'share': 'svg',
    'new-jig': 'png',
    'play-jig': 'svg',
};


@customElement('post-publish-action')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: inline-grid;
                cursor: pointer;
                row-gap: 6px;
                justify-items: center;
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
                transition: color .3s;
                line-height: 1.5;
                font-weight: 600;
                color: #000000;
            }
            :host(:hover) .label {
                color: var(--main-blue);
            }
        `];
    }

    @property()
    kind: Kind = "share";

    render() {
        return html`
            <div class="circle">
                <img-ui path="jig/edit/post-publish/action-${this.kind}.${ext[this.kind]}"></img-ui>
            </div>
            <span class="label">${STR_LABEL_LOOKUP[this.kind]}</span>
        `;
    }
}
