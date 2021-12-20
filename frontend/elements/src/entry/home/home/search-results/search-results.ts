import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

const STR_WE_FOUND = "We found";
const STR_RESULTS = "results";
const STR_FOR = "for";

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

    @property()
    query: string = "";

    @property({ type: Number })
    resultsCount?: number = 0;

    render() {
        return html`
            <div class="main">
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
                <slot name="sections"></slot>
            </div>
        `;
    }
}
