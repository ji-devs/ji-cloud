import { LitElement, html, css, customElement, property, unsafeCSS } from 'lit-element';
import "@elements/core/images/ui";
import { mediaUi } from '@utils/path';

const STR_CREATE_JIG = "Create a New JIG";
const STR_TEMPLATE_PARAGRAPH = "We have created lesson plans you can use for teaching. Create Jig from one of these templates to easily address all your student’s learning needs!";
const STR_RECENT = "My Recent JIGs";

@customElement('jig-gallery')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: grid;
            }
            .width-holder {
                max-width: 1720px;
                margin: 0 auto;
            }
            .main-layer {
                grid-column: 1;
                grid-row: 1;
                display: grid;
                grid-template-rows: 455px auto;
            }
            .top-section {
                background-color: var(--light-blue-3);
            }
            .top-section .width-holder {
                grid-column: 1 / -1;
                padding: 100px;
                display: grid;
                grid-template-columns: auto auto;
                grid-template-rows: min-content min-content;
                justify-content: space-between;
            }
            .create-jig-header {
                font-size: 56px;
                font-weight: 900;
                color: var(--orange);
                margin: 0;
            }
            .template-paragraph {
                font-size: 18px;
                font-weight: 300;
                grid-column: 1;
                margin: 0;
                max-width: 540px;
            }
            .new-jig-section {
                grid-row: 1 / -1;
                grid-column: 2;
                display: grid;
                grid-template-rows: auto auto;
                row-gap: 56px;
                justify-items: center;
            }
            .new-jig-items {
                display: flex;
                column-gap: 32px;
            }
            .bottom-section {
                grid-column: 1 / -1;
                background-image: url(${unsafeCSS(mediaUi('module/_common/jig-gallery/background.png'))});
                background-size: cover;
            }
            .bottom-section .width-holder {
                padding: 100px;
                display: grid;
                row-gap: 48px;
            }
            .recent-top-line {
                display: grid;
                grid-template-columns: auto auto 224px;
                column-gap: 32px;
                align-items: center;
            }
            .recent-header {
                color: var(--dark-blue-4);
                font-size: 40px;
                font-weight: 800;
                margin: 0;
            }
            ::slotted([slot=filters-button]) {
                justify-self: end;
            }
            .recent-items {
                display: grid;
                grid-template-columns: repeat(auto-fill, 230px);
                gap: 64px;
                justify-content: space-between;
            }
            .novel-layer {
                grid-column: 1;
                grid-row: 1;
            }
            .novel-layer .width-holder {
                display: grid;
                grid-template-rows: auto 210px auto 130px auto 1fr;
                grid-template-columns: 473px 225px 8px 145px 1fr;
            }
            .novel-img-1 {
                height: 125px;
                width: 125px;
                grid-row: 1;
                grid-column: 4;
            }
            .novel-img-2 {
                height: 225px;
                width: 225px;
                grid-row: 3 / span 2;
                grid-column: 2;
            }
            .novel-img-3 {
                height: 145px;
                width: 145px;
                grid-row: 4 / span 2;
                grid-column: 4;
            }
      `];
    }

    @property()
    title: string = "";

    render() {
        return html`
            <div class="main-layer">
                <section class="top-section">
                    <div class="width-holder">
                        <h1 class="create-jig-header">${STR_CREATE_JIG}</h1>
                        <p class="template-paragraph">${STR_TEMPLATE_PARAGRAPH}</p>
                        <div class="new-jig-section">
                            <div class="new-jig-items">
                                <slot name="craete-jig"></slot>
                                <slot name="jig-templates"></slot>
                            </div>
                            <div class="see-all-templates-button">
                                <slot name="see-all-templates-button"></slot>
                            </div>
                        </div>
                    </div>
                </section>
                <section class="bottom-section">
                    <div class="width-holder">
                        <div class="recent-top-line">
                            <h2 class="recent-header">${STR_RECENT}</h2>
                            <slot class="filters-button" name="filters-button"></slot>
                            <slot class="search-input" name="search-input"></slot>
                        </div>
                        <div class="recent-items">
                            <slot name="recent-items"></slot>
                        </div>
                    </div>
                </section>
            </div>
            <div class="novel-layer">
                <div class="width-holder">
                    <img-ui class="novel-img-1" path="module/_common/jig-gallery/novel-img-1.png"></img-ui>
                    <img-ui class="novel-img-2" path="module/_common/jig-gallery/novel-img-2.png"></img-ui>
                    <img-ui class="novel-img-3" path="module/_common/jig-gallery/novel-img-3.png"></img-ui>
                </div>
            </div>
        `;
    }
}
