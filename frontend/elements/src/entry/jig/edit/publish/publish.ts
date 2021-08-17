import { LitElement, html, css, customElement, property, unsafeCSS, internalProperty } from 'lit-element';
import "@elements/core/images/ui";

const STR_HEADER_FIRST = "Settings and JIG info.";
const STR_HEADER_SECOND = "Last step before publishing";

@customElement('jig-edit-publish')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: grid;
                place-content: center;
                padding: 50px;
                height: 100%;
                box-sizing: border-box;
            }
            main {
                display: grid;
                place-content: center;
                background-color: var(--white);
                padding: 56px;
                border-radius: 32px;
                box-shadow: 0 3px 8px 0 rgba(0, 0, 0, 0.08);
            }
            .width-holder {
                display: grid;
                grid-template-rows: auto 1fr auto;
                row-gap: 48px;
                max-width: 1300px;
            }
            h1 {
                font-size: 32px;
                font-weight: 900;
                color: var(--dark-blue-4);
                margin: 0;
            }
            h3 {
                font-weight: 500;
                color: #4a4a4a;
                margin: 0;
            }
            .main {
                display: grid;
                grid-template-columns: repeat(4, minmax(auto, 1fr));;
                column-gap: 48px;
                justify-content: center;
                align-items: start;
            }
            ::slotted([slot=img]) {
                display: grid;
                width: 100%;
                border-radius: 16px;
                overflow: hidden;
            }
            ::slotted([slot=public]) {
                display: flex;
                place-content: space-between;
                margin-top: 24px;
                padding: 0 16px;
            }
            .column-2 {
                display: grid;
                align-items: flex-start;
                row-gap: 86px;
            }
            ::slotted([slot=description]) {
                height: 170px;
            }
            .column-3 {
                display: grid;
                align-items: flex-start;
                row-gap: 40px;
            }
            .catagories {
                display: grid;
                row-gap: 16px;
            }
            ::slotted([slot=category-labels]) {
                display: flex;
                flex-wrap: wrap;
                column-gap: 8px;
                row-gap: 12px;
            }
            .additional-resources {
                border-radius: 12px;
                background-color: var(--light-blue-1);
                padding: 16px;
            }
            .additional-resources h4 {
                font-weight: 500;
                margin: 0;
                color: var(--main-blue);
            }
            .additional-resources-items {
                padding: 24px 0;
                display: grid;
                grid-gap: 56px;
            }
            .publish {
                display: grid;
                place-content: center;
            }
      `];
    }


    render() {
        return html`
            <main>
                <div class="width-holder">
                    <div class="header">
                        <h1>${STR_HEADER_FIRST}</h1>
                        <h3>${STR_HEADER_SECOND}</h3>
                    </div>
                    <div class="main">
                        <div  class="column-1">
                            <slot name="img"></slot>
                            <div class="public">
                                <slot name="public"></slot>
                            </div>
                        </div>
                        <div class="column-2">
                            <slot name="name"></slot>
                            <slot name="description"></slot>
                        </div>
                        <div class="column-3">
                            <slot name="language"></slot>
                            <slot name="age"></slot>
                            <slot name="goal"></slot>
                            <div class="catagories">
                                <slot name="catagories-select"></slot>
                                <slot name="category-labels"></slot>
                            </div>
                        </div>
                        <div class="column-4 additional-resources">
                            <h4>Additional resources (Coming soon!)</h4>
                            <div class="additional-resources-items">
                                <slot name="additional-resources"></slot>
                            </div>
                        </div>
                    </div>
                    <div class="publish">
                        <slot name="publish"></slot>
                    </div>
                </div>
            </main>
        `;
    }
}
