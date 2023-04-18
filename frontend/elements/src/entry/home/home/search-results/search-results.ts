import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing, TemplateResult } from "lit-html";

const STR_WE_FOUND = "We found";
const STR_NONE_FOUND = "Oh snap! We couldn't find any matches";

const STR_FOR = "for";

const STR_LOADING = "So many great JIGs and resources to sift through...";

const KINDS: {[key: string]: string[]} = {
    jig: ["JIG", "JIGs"],
    course: ["Course", "Courses"],
    resource: ["Resource", "Resources"],
};
const STR_AND = "and";

@customElement("home-search-results")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    /* padding: 50px 0; */
                }
                h1 {
                    font-size: 24px;
                    color: #383838;
                    text-align: center;
                    font-weight: normal;
                }
                h1 a {
                    color: #fd7076;
                    font-weight: 600;
                }

                h1 a {
                    text-decoration: none;
                }

                .algolia {
                    display: grid;
                    justify-content: center;
                    margin-top: 50px;
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
    courseCount: number = 0;

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

        addResultCount("jig", this.jigCount);
        addResultCount("course", this.courseCount);
        addResultCount("resource", this.resourceCount);

        // If we're rendering more than one set of results then add STR_AND
        // before the last result.
        // If we're rendering three set of results then add a comma after
        // the first result.
        if (results.length >= 3) {
            results.splice(1, 0, html`, `);
        }
        if (results.length >= 2) {
            results.splice(-1, 0, html` ${STR_AND} `);
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
                : this.jigCount + this.courseCount + this.resourceCount > 0
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
