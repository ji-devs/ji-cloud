import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

const STR_WE_FOUND = "We found";
const STR_NONE_FOUND = "Oh snap! We couldn't find any matches";

const STR_RESULTS = "results";
const STR_FOR = "for";

const STR_LOADING = "So many great JIGs and resources to sift through...";

@customElement("home-search-results")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    padding: 50px 0;
                }
                h1 {
                    font-size: 40px;
                    color: #383838;
                    text-align: center;
                    font-weight: normal;
                }
                h1 .results-count,
                h1 .query {
                    color: #fd7076;
                    font-weight: bold;
                }
            `,
        ];
    }

    @property({ type: Boolean })
    loading: boolean = false;

    @property()
    query: string = "";

    @property({ type: Number })
    resultsCount?: number = 0;

    renderResultsFound() {
        return html`
            <h1>
                ${STR_WE_FOUND}
                <span class="results-count">${this.resultsCount}</span>
                ${STR_RESULTS}
                ${
                    this.query.trim() !== "" ? html`
                        ${STR_FOR}
                        <span class="query">${this.query}</span>
                    ` : nothing
                }
            </h1>
        `;
    }

    renderNoResultsFound() {
        return html`
            <h1>
                ${STR_NONE_FOUND}
                ${
                    this.query.trim() !== "" ? html`
                        ${STR_FOR}
                        <span class="query">${this.query}</span>
                    ` : nothing
                }
            </h1>
        `;
    }

    renderLoading() {
        return html`
            <h1>${STR_LOADING}</h1>
        `;
    }

    render() {
        return html`
            ${this.loading
                ? this.renderLoading()
                : this.resultsCount
                    ? this.renderResultsFound()
                    : this.renderNoResultsFound()
            }
            <div class="main">
                <slot name="sections"></slot>
            </div>
        `;
    }
}
