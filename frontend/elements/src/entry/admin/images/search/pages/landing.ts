import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import "@elements/core/titles/variants/horizontal-underlined-title";
import "@elements/core/pagination/widget";
import "@elements/core/inputs/composed/dropdown-underlined";
import "@elements/entry/admin/images/search/image-cell";
import "@elements/entry/admin/images/search/publish-filter";

const STR_TITLE = "Search Images";
const STR_FOUND_1 = "We found";
const STR_FOUND_2 = "images for";
const STR_ADD = "Add image";

@customElement('image-search')
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
            aside .title {
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

            aside .right {
                display: flex;
                align-items: center;
                gap: 24px;
            }

            :host {
                display: block;
                margin-top: 29px;
                padding-left: 40px;
                padding-right: 40px;
            }
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
              --width: 200px;
          }
    `];
  }

    gotoRoute(route: string) {
        this.dispatchEvent(
            new CustomEvent("custom-route", {
                detail: { route },
                composed: true,
                bubbles: true
            })
        );
    }
  @property({type: Number})
  nResults: number = 0;

  @property()
  query: string = "";

  render() {

      const {nResults, query} = this;

      const hasSearched = nResults != 0; 

    return html`
            <aside>
                <div class="title">${STR_TITLE}</div>
                <div class="right">
                    <button-rect-icon @click=${() => this.gotoRoute("add")} color="blue" size="small" iconBefore="plus">${STR_ADD}</button-rect-icon>
                    <input-search .value=${query}></input-search>
                </div>
            </aside>
            <article>
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
            </article>
  `;
  }
}
