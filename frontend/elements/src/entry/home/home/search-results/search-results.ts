import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing, TemplateResult } from "lit-html";

const STR_WE_FOUND = "We found";
const STR_NONE_FOUND = "Oh snap! We couldn't find any matches";

const STR_FOR = "for";

const STR_LOADING = "So many great JIGs and resources to sift through...";

const KINDS: {[key: string]: string[]} = {
    jigs: ["JIG", "JIGs"],
    resources: ["Resource", "Resources"],
};
const STR_AND = "and";

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
                h1 a {
                    color: #fd7076;
                    font-weight: bold;
                }

                h1 a {
                    text-decoration: none;
                }

                .algolia {
                    display: grid;
                    justify-content: center;
                    margin-top: 50px;
                }

                /* mobile */
                @media (max-width: 1000px) {
                    h1 {
                        font-size: 20px;
                    }
                }
            `,
        ];
    }

    @property({ type: Boolean })
    loading: boolean = false;

    @property()
    query: string = "";

    @property({ type: Number })
    jigCount: number = 0;

    @property({ type: Number })
    resourceCount: number = 0;

    private scrollToResults(event: MouseEvent) {
        event.preventDefault();
        const slot = this.shadowRoot!.querySelector("slot") as HTMLSlotElement;

        const target = event.target as HTMLAnchorElement;
        const kind = target.getAttribute("href")!.replace(/^#/, "");

        slot.assignedElements().forEach((element) => {
            if (element.getAttribute("kind") === kind) {
                element.scrollIntoView({
                    behavior: "smooth",
                });
            }
        })
    }

    renderResultsFound() {
        let results: TemplateResult[] = [];

        const addResultCount = (kind: string, count: number) => {
            if (count > 0) {
                let noun = BigInt(count) === BigInt(1) ? KINDS[kind][0] : KINDS[kind][1];
                results.push(html`<a @click="${this.scrollToResults}" href="#${kind}">${count} ${noun}</a>`);
            }
        }

        addResultCount("jigs", this.jigCount);
        addResultCount("resources", this.resourceCount);
        if (this.jigCount > 0 && this.resourceCount > 0) {
            // If we're rendering both sets of results then add STR_AND between
            // them.
            // NOTE: If at some point we want to include more than two results
            // sections, then we will need to update this logic to handle it.
            results.splice(1, 0, html` ${STR_AND} `);
        }

        return html`
            <h1>
                ${STR_WE_FOUND}
                ${results}
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
                : this.jigCount + this.resourceCount > 0
                    ? this.renderResultsFound()
                    : this.renderNoResultsFound()
            }
            <div class="main">
                <slot name="sections"></slot>
            </div>
            <div class="algolia">
                <a href="https://www.algolia.com/" target="_blank">
                    <img-ui path="search-by-algolia.png"></img-ui>
                </a>
            </div>
        `;
    }
}
