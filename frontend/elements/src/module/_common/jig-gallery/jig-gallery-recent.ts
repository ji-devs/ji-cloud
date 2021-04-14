import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from 'lit-html';
import '@elements/core/images/ui';
import '@elements/core/menu/kebab';
import '@elements/core/menu/menu-line';
import { styles } from './styles';

const STR_DRAFT = "Draft";

@customElement('jig-gallery-recent')
export class _ extends LitElement {
    static get styles() {
        return [
            styles,
            css`
                :host {
                    cursor: pointer;
                }
                .draft {
                    padding: 6px 8px;
                    border-radius: 6px;
                    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.16);
                    background-color: #ffea79;
                    grid-column: 2;
                    height: 32px;
                    color: var(--dark-gray-5);
                    font-size: 14px;
                    font-weight: 600;
                    line-height: 1.5;
                    box-sizing: border-box;
                    margin-top: -8px;
                    margin-left: 16px;
                    position: absolute;
                    display: inline-flex;
                    align-items: center;
                    column-gap: 4px;
                }
                .card {
                    display: inline-grid;
                    background-color: #fff;
                    height: 230px;
                    grid-template-rows: 130px auto;
                }
                .top-section {
                    display: grid;
                    grid-template-rows: 16px auto 1fr;
                    grid-template-columns: 1fr auto 16px;
                }
                .top-section img-ui {
                    grid-column: 1 / -1;
                    grid-row: 1 / -1;
                }
                .menu {
                    display: none;
                    grid-column: 2;
                    grid-row: 2;
                }
                :host(:hover) .menu {
                    display: block;
                }
                .bottom-section {
                    padding: 16px;
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    align-content: space-between;
                }
                .label {
                    grid-column: 1 / -1;
                    text-align: center;
                }
                .last-edited {
                    font-size: 14px;
                    font-weight: 500;
                    text-align: right;
                }
            `
        ];
    }

    @property()
    label: string = "";

    @property()
    img: string = "";

    @property({type: Boolean, reflect: true})
    draft = false;

    @property()
    ages: string = "";

    @property()
    lastEdited: string = "";

    render() {
        return html`
            ${ this.draft ? (
                html`<div class="draft">
                    <img-ui path="module/_common/jig-gallery/draft-icon.svg"></img-ui>
                    <span>${STR_DRAFT}</span>
                </div>`
            ) : nothing }
            <div class="card">
                <div class="top-section">
                    <img-ui path="${this.img}"></img-ui>
                    <menu-kebab class="menu">
                        <slot name="menu-content" slot="menu-content"></slot>
                    </menu-kebab>
                </div>
                <div class="bottom-section">
                    <span class="label main-text">${this.label}</span>
                    <span class="ages">
                        <img-ui path="module/_common/jig-gallery/age-icon${this.draft ? "-draft" : ""}.svg"></img-ui>
                        ${this.ages}
                    </span>
                    <span class="last-edited">${this.lastEdited}</span>
                </div>
            </div>
        `;
    }
}
