import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/ji";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/inputs/search";
import { nothing } from "lit-html";

@customElement('image-page')
export class _ extends LitElement {
    static get styles() {
        return [css`
            aside {
                display: flex;
                justify-content: space-between;
                border-bottom: solid 1px #e5e7ef;
                padding-bottom: 29px;
                margin-bottom: 29px;
            }
            .title {
                font-size: 24px;
                font-weight: 300;
                font-stretch: normal;
                font-style: normal;
                line-height: 1.25;
                letter-spacing: -0.24px;
                text-align: left;
                color: #000000;
            }

            :host {
                    display: block;
                margin-top: 29px;
                padding-left: 40px;
                padding-right: 40px;
            }
            article {
            }

    `];
    }

    @property()
    title:string = "";

    render() {
        const {title} = this;

        return html`
            <aside>
                <div class="title">${title}</div> 
                <input-search></input-search>
            </aside>
            <article>
                <slot></slot>
            </article>
  `;
    }
}
