import { LitElement, html, css, customElement, property } from 'lit-element';

export type Kind = "jigs" | "learning-paths";

const STR_JIGS = "JIGs";

@customElement('home-search-results-section')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: grid;
                row-gap: 48px;
                padding: 0 50px;
                max-width: 1800px;
                margin: 0 auto;
            }
            .top-line {
                display: flex;
                justify-content: space-between;
                align-items: center;
            }
            .left-side {
                display: flex;
                position: relative;
            }
            .left-side img-ui {
                position: absolute;
                right: 100%;
            }
            h2 {
                margin: 0;
                font-size: 40px;
                font-weight: 800;
                color: var(--dark-blue-4);
            }
            .results-count {
                font-size: 24px;
                font-weight: 500;
            }
            .results {
                /* display: flex;
                flex-wrap: wrap; */

                display: grid;
                grid-template-columns: repeat(auto-fill, 354px);

                justify-content: space-between;
                row-gap: 80px;
                column-gap: 40px;
            }
            .load-more {
                display: grid;
                place-content: center;
            }
        `];
    }

    @property({ reflect: true })
    kind: Kind = "jigs";

    @property({ type: Number })
    resultsCount: number = 0;

    render() {
        return html`
            <div class="top-line">
                <div class="left-side">
                    <img-ui path="entry/home/search-results/${this.kind === "jigs" ? "jig-section.png" : "learning-paths.svg"}"></img-ui>
                    <h2>
                        ${STR_JIGS}
                        <span class="results-count">(${this.resultsCount})</span>
                    </h2>
                </div>
                <slot name="sort"></slot>
            </div>
            <div class="results">
                <slot name="results"></slot>
            </div>
            <div class="load-more">
                <slot name="load-more"></slot>
            </div>
        `;
    }
}
