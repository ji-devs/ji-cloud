import { LitElement, html, css, customElement, property } from 'lit-element';

export type ProgressColor = 'blue' | 'green';

@customElement('progress-bar')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host([color=blue]) {
                --color: var(--main-blue);
                --background-color: #c4dbfe;
            }
            :host([color=green]) {
                --color: #7fd29c;
                --background-color: #d0ebda;
            }
            .outer {
                display: grid;
                background-color: var(--background-color);
                height: 24px;
                border-radius: 12px;
                grid-template-columns: 24px repeat(99, 1fr);
            }
            .inner {
                border-radius: 12px;
                background-color: var(--color);
            }
        `];
    }

    @property({type: Number})
    progress: number = 100;

    @property()
    color: ProgressColor = 'blue';

    render() {
        return html`
            <style>
                .inner {
                    grid-column: 1 / span ${ this.progress + 1 };
                }
            </style>
            <div class="outer">
                <div class="inner"></div>
            </div>
        `;
    }
}
