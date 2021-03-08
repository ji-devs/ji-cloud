import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/ji";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/inputs/search";
import { nothing } from "lit-html";

const STR_ADD = "Add image";

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
                margin-right: 10px;
            }

            .right {
                display: flex;
                gap: 24px;
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

    gotoAdd() {
        this.dispatchEvent(
            new CustomEvent("custom-route", {
                detail: { route: "add"},
                composed: true,
                bubbles: true
            })
        );
    }
    @property()
    title:string = "";

    @property({type: Boolean})
    hideAdd:boolean = false;

    @property()
    query:string = "";

    render() {
        const {title, hideAdd, query} = this;

        return html`
            <aside>
                <div class="title">${title}</div>
                <div class="right">
                    ${hideAdd ? nothing : html`<button-rect @click=${this.gotoAdd} color="blue" size="small" iconBefore="plus">${STR_ADD}</button-rect>`}
                    <input-search .value=${query}></input-search>
                </div>
            </aside>
            <article>
                <slot></slot>
            </article>
  `;
    }
}
