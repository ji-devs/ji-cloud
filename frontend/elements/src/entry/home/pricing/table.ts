import { LitElement, html, css, customElement, property, TemplateResult } from "lit-element";
import { nothing } from "lit-html";

export type Kind = 'individuals' | 'schools';

@customElement("pricing-table")
export class _ extends LitElement {
    static get styles() {
        return [css`
            .table-wrapper {
                display: grid;
                place-content: center;
            }
            .table {
                border-collapse: collapse;
                font-weight: 400px;
                font-size: 13px;
                line-height: 1.4;
                display: grid;
            }
            @media (min-width: 1024px) {
                .table {
                    margin: 0 32px;
                }
            }
            :host([kind=individuals]) .table {
                grid-template-columns: repeat(3, 1fr);
            }
            @media (min-width: 1024px) {
                :host([kind=individuals]) .table {
                    grid-template-columns: minmax(286px, 316px) repeat(3, minmax(224px, 250px));
                }
                :host([kind=schools]) .table {
                    grid-template-columns: minmax(365px, 365px) minmax(595px, 595px);
                }
            }
            .row {
                display: contents;
            }
            .row.begin-row .cell {
                padding-top: 16px;
                    /* border-top: none; */
            }
            @media (min-width: 1024px) {
                .row.begin-row .cell {
                    border-top: solid 1px var(--light-gray-1);
                }
            }
            .row.end-row .cell {
                padding-bottom: 16px;
            }
            .row.blue .cell {
                background-color: var(--light-blue-1);
            }
            .cell {
                padding: 18px 16px;
                text-align: center;
                border-left: solid 1px var(--light-gray-1);
                padding: 8px 16px;
                vertical-align: top;
            }
            .cell:not(.cell-header) {
                display: inline-grid;
                place-content: center;
            }
            .cell.empty {
                display: none;
            }
            @media (min-width: 1024px) {
                .cell.empty {
                    display: inline;
                }
            }
            .cell:first-child {
                text-align: left;
            }
            .cell:last-child {
                /* border-right: none; */
            }
            @media (min-width: 1024px) {
                .cell:last-child {
                    border-right: solid 1px var(--light-gray-1);
                }
            }
            .row:last-child .cell {
                border-bottom: solid 1px var(--light-gray-1);
            }
            .cell.cell-header {
                grid-column: 1 / -1;
                font-size: 14px;
                font-weight: 600;
                /* text-align: left; */
                /* padding-top: 24px!important; */
            }
            @media (min-width: 1024px) {
                .cell.cell-header {
                    grid-column: 1;
                    font-size: inherit;
                    font-weight: inherit;
                    text-align: center;
                    /* padding-top: 24px!important; */
                }
                /* .cell:not(.cell-header) {
                    padding-bottom: 24px;
                } */
            }
            .cell h4 {
                font-size: 16px;
                font-weight: 700;
                color: var(--dark-blue-4);
                margin: 0;
            }
            .cell h4 .coming-soon {
                border-radius: 4px;
                background-color: var(--light-red-4);
                font-size: 10px;
                font-weight: 600;
                color: #ffffff;
                padding: 0px 4px;
                margin-left: 10px;
            }
            .cell p {
                margin: 0;
            }
            .school-only-row {
                font-weight: 600;
            }
            .school-only-row .cell {
                grid-template-columns: 1fr auto 1fr;
                justify-items: start;
                gap: 16px;
            }
            .school-only-row .cell fa-icon {
                grid-column: 2;
            }
            .school-only-row .school-only-label {
                font-size: 12px;
                font-weight: bold;
                color: #149546;
                border-radius: 12px;
                border: solid 1px #149647;
                padding: 3px 8px;
            }
            .individuals-desktop-price {
                display: none;
            }
            @media (min-width: 1024px) {
                :host([kind=individuals]) .individuals-desktop-price {
                    display: contents;
                }
                .individuals-desktop-price .cell {
                    color: var(--dark-gray-6);
                }
            }
            .individuals-desktop-price .cell.message {
                padding-bottom: 0;
            }
            .individuals-desktop-price .plan-name {
                margin: 0;
                font-size: 16px;
                font-weight: 700;
            }
            .individuals-desktop-price .plan-price {
                margin: 0;
                margin-top: 30px;
                font-size: 38px;
                font-weight: 700;
            }
            .individuals-desktop-price .frequency {
                margin: 0;
                font-size: 13px;
                font-weight: 400;
            }
            .individuals-desktop-price button {
                margin-top: 35px;
            }
            .custom-subscription, table {
                max-width: 1100px;
                margin: 0 32px;
            }
            .custom-subscription {
                margin-top: 24px;
            }
        `];
    }

    @property({ reflect: true })
    kind: Kind = "individuals";

    @property({ reflect: true })
    frequency?: string;

    render() {
        return html`
            <div class="table-wrapper">
                <div class="table">
                    <div class="row begin-row end-row white individuals-desktop-price">
                        <div class="cell message">
                            <slot name="pricing-message"></slot>
                        </div>
                        <div class="cell">
                            <h5 class="plan-name">Free</h5>
                            <h3 class="plan-price">$0.00</h3>
                            <slot name="free-action"></slot>
                        </div>
                        <div class="cell">
                            <h5 class="plan-name">Basic</h5>
                            <h3 class="plan-price">$14.99</h3>
                            <p class="frequency">${this.frequency}</p>
                            <slot name="basic-action"></slot>
                        </div>
                        <div class="cell">
                            <h5 class="plan-name">Pro</h5>
                            <h3 class="plan-price">$29.99</h3>
                            <p class="frequency">${this.frequency}</p>
                            <slot name="pro-action"></slot>
                        </div>
                    </div>
                    <div class="row begin-row blue">
                        <div class="cell cell-header"><h4>Recommended for</h4></div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell">Access to FREE content that is a productive screen-time alternative. Interactive. Educational. Fun.</div>
                                <div class="cell">Parents and educators who want full-access to high-quality educational content for students.</div>
                                <div class="cell">Full-access educators who also want to create high-quality lessons with their own material.</div>
                            `
                        })}
                    </div>
                    <div class="row end-row blue">
                        <div class="cell empty"></div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><button-rect kind="text" color="blue">Learn more</button-rect></div>
                                <div class="cell"><button-rect kind="text" color="blue">Learn more</button-rect></div>
                                <div class="cell"><button-rect kind="text" color="blue">Learn more</button-rect></div>
                            `
                        })}
                    </div>
                    <div class="row begin-row white">
                        <div class="cell cell-header"><h4>Content</h4></div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell empty"></div>
                            `,
                            individuals: html`
                                <div class="cell empty"></div>
                                <div class="cell empty"></div>
                                <div class="cell empty"></div>
                            `
                        })}
                    </div>
                    <div class="row white">
                        <div class="cell cell-header">Unlimited access to JIGs, playlists, & resources</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell">FREE content only</div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <div class="row end-row white">
                        <div class="cell cell-header">Create a playlist of Jigzi content</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <div class="row begin-row blue">
                        <div class="cell cell-header"><h4>Creation</h4></div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell empty"></div>
                            `,
                            individuals: html`
                                <div class="cell empty"></div>
                                <div class="cell empty"></div>
                                <div class="cell empty"></div>
                            `
                        })}
                    </div>
                    <div class="row blue">
                        <div class="cell cell-header">Create your own JIGs</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell">Limited to 5 JIGs</div>
                                <div class="cell">Limited to 5 JIGs</div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <div class="row blue">
                        <div class="cell cell-header">Full access to ALL images & themes</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell">FREE images & themes</div>
                                <div class="cell">FREE images & themes</div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <div class="row blue">
                        <div class="cell cell-header">Print images, posters & flashcards</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <div class="row blue">
                        <div class="cell cell-header">Upload & share your own resources</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell">Limited to 5 resources</div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <!-- <div class="row end-row blue">
                        <div class="cell cell-header">Create courses with embedded links & videos</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                            <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div> -->
                    <div class="row begin-row white">
                        <div class="cell cell-header"><h4>ClassMate<span class="coming-soon">Coming Soon</span></h4></div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell empty"></div>
                            `,
                            individuals: html`
                                <div class="cell empty"></div>
                                <div class="cell empty"></div>
                                <div class="cell empty"></div>
                            `
                        })}
                    </div>
                    <div class="row white">
                        <div class="cell cell-header">Collect & organize your favorite Jigzi content</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <div class="row white">
                        <div class="cell cell-header">Monitor student success through scoring</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <div class="row end-row white">
                        <div class="cell cell-header">Create a JIGZONE for students to create their own JIGs!</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <div class="row begin-row blue">
                        <div class="cell cell-header"><h4>Community</h4></div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell empty"></div>
                                <div class="cell empty"></div>
                                <div class="cell empty"></div>
                            `
                        })}
                    </div>
                    <div class="row end-row blue">
                        <div class="cell cell-header">A community profile page displaying your Jigzi creations</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <div class="row end-row blue">
                        <div class="cell cell-header">Follow favorite creators & join circles to build your personal community</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <!-- <div class="row end-row blue">
                        <div class="cell cell-header">Access courses for professional development</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div> -->
                    <div class="row end-row blue">
                        <div class="cell cell-header">Create courses for professional development</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-dash"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    ${ this.kind === "schools" ? html`
                        <div class="row end-row blue school-only-row">
                            <div class="cell cell-header">School community page for all your staff creations!</div>
                            <div class="cell">
                                <fa-icon icon="fa-solid fa-check"></fa-icon>
                                <span class="school-only-label">Just for Schools!</span>
                            </div>
                        </div>
                    ` : nothing }
                    <div class="row begin-row white">
                        <div class="cell cell-header"><h4>Support</h4></div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell empty"></div>
                                <div class="cell empty"></div>
                                <div class="cell empty"></div>
                            `
                        })}
                    </div>
                    <div class="row white">
                        <div class="cell cell-header">Jigzi tutorials & webinars</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    <div class="row end-row white">
                        <div class="cell cell-header">Account support</div>
                        ${cells(this.kind, {
                            schools: html`
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `,
                            individuals: html`
                                <div class="cell">via email</div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                                <div class="cell"><fa-icon icon="fa-solid fa-check"></fa-icon></div>
                            `
                        })}
                    </div>
                    ${ this.kind === "schools" ? html`
                        <div class="row end-row blue school-only-row">
                            <div class="cell cell-header">Professional platform training by Jigzi experts</div>
                            <div class="cell">
                                <fa-icon icon="fa-solid fa-check"></fa-icon>
                                <span class="school-only-label">Just for Schools!</span>
                            </div>
                        </div>
                    ` : nothing }
                </div>
                <p class="custom-subscription">
                    Can’t find a subscription that fits your needs?
                    <button-rect kind="text" color="blue">Contact us</button-rect>
                </p>
            </div>
        `;
    }
}

interface CellsConfig {
    individuals: TemplateResult;
    schools: TemplateResult;
}
function cells(kind: Kind, config: CellsConfig): TemplateResult {
    return kind === "individuals" ? config.individuals : config.schools;
}
