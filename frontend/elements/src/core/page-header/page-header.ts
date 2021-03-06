import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('page-header')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                height: 88px;
                display: grid;
                grid-template-columns: auto auto 1fr auto;
                align-items: center;
                padding: 0 40px;
            }
            nav {
                display: flex;
                column-gap: 8px;
                height: 100%;
            }
            .donate {
                display: grid;
                place-content: center;
            }
            .user {
                display: flex;
                column-gap: 16px;
            }
        `];
    }

    render() {
        return html`
            <img-ui class="logo" path="core/page-header/logo.svg"></img-ui>
            <nav>
                <slot name="links"></slot>
            </nav>
            <div class="donate">
                <slot name="donate"></slot>
            </div>
            <div class="user">
                <slot name="user"></slot>
            </div>
        `;
    }
}
