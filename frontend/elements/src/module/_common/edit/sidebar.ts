import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('module-sidebar')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: grid;
                grid-template-rows: 160px 1fr;
                height: 100%;
                width: 556px;
                box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                background-color: #f6fafe;
            }
            .nav {
                padding-top: 40px;
                padding-left: 50px;
                padding-right: 50px;
            }
            .content {
                display: flex;
                flex-direction: column;
                width: 100%;
                height: 100%;
                overflow-y: auto;
            }
        `];
    }

    render() {
        return html`
            <div class="nav">
                <slot name="nav"></slot>
            </div>
            <article class="content">
                <slot name="content"></slot>
            </article>
        `;
    }
}
