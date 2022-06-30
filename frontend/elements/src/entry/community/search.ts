import { LitElement, html, css, customElement, property, query } from "lit-element";

const STR_WE_FOUND = "We found";
const STR_AND = "and";
const STR_FOR = "for";
const STR_MEMBERS = "Members";
const STR_CIRCLES = "Circles";

@customElement("community-search")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    align-items: center;
                    padding: 40px 30px;
                    display: grid;
                    row-gap: 24px;
                }
                h2 {
                    text-align: center;
                    color: var(--dark-gray-6);
                    font-size: 24px;
                    font-weight: 500;
                    margin: 0;
                }
                h2 a {
                    font-weight: 600;
                    color: var(--main-red);
                    cursor: pointer;
                }
                section {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    row-gap: 20px;
                }
                section h4 {
                    font-size: 20px;
                    font-weight: bold;
                    color: var(--dark-blue-4);
                    margin: 0;
                }
                section .items {
                    display: grid;
                    row-gap: 24px;
                    grid-column: 1 / -1;
                }
                section ::slotted([slot$=see-more]) {
                    justify-self: center;
                    grid-column: 1 / -1;
                }
            `,
        ];
    }

    @property()
    query: string = "";

    @property({ type: Number })
    memberCount: number = 0;

    @property({ type: Number })
    circleCount: number = 0;

    @query("#members")
    private membersSection!: HTMLElement;

    @query("#circles")
    private circlesSection!: HTMLElement;

    scrollToMembers() {
        this.membersSection.scrollIntoView({behavior: "smooth"});
    }

    scrollToCircles() {
        this.circlesSection.scrollIntoView({behavior: "smooth"});
    }

    render() {
        return html`
            <h2>
                ${ STR_WE_FOUND }
                <a @click=${this.scrollToMembers}>
                    ${ this.memberCount }
                    ${ STR_MEMBERS }
                </a>
                ${ STR_AND }
                <a @click=${this.scrollToCircles}>
                    ${ this.circleCount }
                    ${ STR_CIRCLES }
                </a>
                ${ STR_FOR }
                "${ this.query }"
            </h2>
            <section id="members">
                <h4>
                    ${ STR_MEMBERS }
                    (${ this.memberCount })
                </h4>
                <slot name="members-sort"></slot>
                <div class="items">
                    <slot name="members"></slot>
                </div>
                <slot name="members-see-more"></slot>
            </section>
            <section id="circles">
                <h4>
                    ${ STR_CIRCLES }
                    (${ this.circleCount })
                </h4>
                <slot name="circles-sort"></slot>
                <div class="items">
                    <slot name="circles"></slot>
                </div>
                <slot name="circles-see-more"></slot>
            </section>
        `;
    }
}
