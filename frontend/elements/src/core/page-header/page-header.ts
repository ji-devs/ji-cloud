import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('page-header')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                height: 88px;
                display: grid;
                grid-template-columns: repeat(5, auto);
                justify-content: space-between;
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
                height: 100%;
                align-items: center;
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
            <div class="student-code">
                <slot name="student-code"></slot>
            </div>
            <div class="user">
                <slot name="user"></slot>
            </div>
        `;
    }
}
