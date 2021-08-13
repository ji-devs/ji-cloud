import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/entry/admin/images/base-page";
import "@elements/core/titles/ji";
import "@elements/core/inputs/composed/search";
import { nothing } from "lit-html";

const STR_GENERAL_SUMMARY = "General Summary";
const STR_IMAGENAME = "Image name";
const STR_DESCRIPTIONTITLE = "Image description";
const STR_STYLETITLE = "Image style";
const STR_AGE = "Suitable for age";
const STR_STREAM = "Suitable for jewish stream";
const STR_TAGS = "Tags (internal use only)";
const STR_DATE_TIME = "Date and time (automatic)";
//FUTURE: const STR_USED = "To be used only for";

const STR_CATEGORIES_REPORT = "Categories summary";
@customElement('image-meta-section-3')
export class _ extends LitElement {
  static get styles() {
    return [css`
   
          :host {
              display: grid;

              grid-template-columns: 704px 1fr;
              gap: 10px;
          }
          .label-info {
          }
          .summary-line {
              display: flex;
              justify-content: space-between;
          }
          header {
              font-size: 16px;
              font-weight: 500;
              font-stretch: normal;
              font-style: normal;
              line-height: 1.25;
              letter-spacing: -0.16px;
              text-align: left;
              color: #5590fc;
              margin-bottom: 10px;
          }

          .summary {
              margin-bottom: 20px;
          }
    `];
  }

  render() {

    return html`
        <div class="label-info">
            <div class="summary-line">
                <header>${STR_GENERAL_SUMMARY}</header>
                <slot name="edit"></slot>
            </div>
                <card-blue>
                    <title-ji color="blue" class="summary-title">${STR_IMAGENAME}</title-ji>
                    <div class="summary">
                        <slot name="imagename"></slot>
                    </div>
                    <title-ji color="blue" class="summary-title">${STR_DESCRIPTIONTITLE}</title-ji>
                    <div class="summary">
                        <slot name="description"></slot>
                    </div>
                    <title-ji color="blue" class="summary-title">${STR_STYLETITLE}</title-ji>
                    <div class="summary">
                        <slot name="style"></slot>
                    </div>
                    <title-ji color="blue" class="summary-title">${STR_AGE}</title-ji>
                    <div class="summary">
                        <slot name="age"></slot>
                    </div>
                    <title-ji color="blue" class="summary-title">${STR_STREAM}</title-ji>
                    <div class="summary">
                        <slot name="stream"></slot>
                    </div>
                    <title-ji color="blue" class="summary-title">${STR_TAGS}</title-ji>
                    <div class="summary">
                        <slot name="tags"></slot>
                    </div>
                    <title-ji color="blue" class="summary-title">${STR_DATE_TIME}</title-ji>
                    <div class="summary">
                        <slot name="date-time"></slot>
                    </div>
                </card-blue>
        </div>

        <div>
            <header>${STR_CATEGORIES_REPORT}</header>
            <card-blue>
                <slot name="category-report"></slot>
            </card-blue>
        </div>
  `;
  }
}


