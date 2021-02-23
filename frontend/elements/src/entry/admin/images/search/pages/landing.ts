import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import "@elements/core/titles/variants/horizontal-underlined-title";
import "@elements/entry/admin/images/base-page";
import "@elements/core/pagination/widget";
import "@elements/core/inputs/dropdowns/dropdown-underlined";
import "@elements/entry/admin/images/search/image-cell";
import "@elements/entry/admin/images/search/publish-filter";

const STR_TITLE = "Search Images";
const STR_FOUND_1 = "We found";
const STR_FOUND_2 = "images for";

@customElement('image-search')
export class _ extends LitElement {
  static get styles() {
      return [css`
          header {
              display: flex;
              justify-content: space-between;
          }

          .images {
              display: flex;
              gap: 61px;
              flex-wrap: wrap;
              margin: 30px 0;
          }

          footer.visible {
              display: flex;
              justify-content: center;
          }

          .pagination {
              display: none;
          }

          .resultsText {
              display: none;
              font-size: 18px;
              font-weight: 500;
              font-stretch: normal;
              font-style: normal;
              line-height: 1.28;
              letter-spacing: -0.18px;
          }

          .visible {
              display: block;
          }

          .highlight {
              color: #5590fc;
          }
          .highlight:before { content: "\\00a0 "; }
          .highlight:after { content: "\\00a0 "; }

          .publish-filter {
              width: 200px;
          }
    `];
  }

  @property({type: Number})
  nResults: number = 0;

  @property()
  query: string = "";

  render() {

      const {nResults, query} = this;

      const hasSearched = nResults != 0; 

    return html`
        <image-page title="${STR_TITLE}">
            <header>
                <div class="${classMap({resultsText: true, visible: hasSearched})}">
                    ${STR_FOUND_1}<span class="highlight">${nResults}</span>${STR_FOUND_2}<span class="highlight">${query}</span>
                </div>
                <div class="${classMap({pagination: true, visible: hasSearched})}">
                    <slot name="pagination-top"></slot>
                </div>
                <div class="publish-filter">
                    <slot name="publish-filter"></slot>
                </div>
            </header>
            <div class="images">
                <slot name="images"></slot>
            </div>
            <footer class="${classMap({pagination: true, visible: hasSearched})}">
                <slot name="pagination-bottom"></slot>
            </footer>
        </image-page>
  `;
  }
}
