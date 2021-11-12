import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import "@elements/core/titles/ji";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/inputs/composed/search";
import "@elements/core/buttons/rectangle-icon";
import "@elements/core/buttons/rectangle";
import { nothing } from "lit-html";

export type SECTION = "general" | "categories" | "summary";

const STR_ADD = "Add image";
const STR_TITLE = "Label Images";
const STR_PUBLISH = "Publish";

const STR_SECTION_LABEL_LOOKUP: { [key in SECTION]: string } = {
    general: "General",
    categories: "Categories",
    summary: "Summary",
};

@customElement("image-meta-header")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                aside {
                    display: flex;
                    justify-content: space-between;
                    border-bottom: solid 1px #e5e7ef;
                    padding-bottom: 29px;
                    margin-bottom: 29px;
                }
                .left {
                    display: flex;
                    align-items: center;
                    gap: 24px;
                }

                .tabs {
                    display: flex;
                    gap: 8px;
                    font-size: 18px;
                    font-weight: 300;
                    font-stretch: normal;
                    font-style: normal;
                    line-height: 1.28;
                    letter-spacing: -0.18px;
                    text-align: left;
                    color: #000;
                }

                .tab {
                    cursor: pointer;
                }

                .tabSelected {
                    color: var(--main-blue);
                }

                .title {
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

                .right {
                    display: flex;
                    align-items: center;
                    gap: 24px;
                }
            `,
        ];
    }

    gotoRoute(route: string) {
        this.dispatchEvent(
            new CustomEvent("custom-route", {
                detail: { route },
                composed: true,
                bubbles: true,
            })
        );
    }
    @property()
    title: string = "";

    @property()
    query: string = "";

    @property()
    section: SECTION = "general";

    render() {
        const { query, section } = this;

        const renderTab = (target: SECTION, decorate: boolean) => {
            return html`
                <div
                    class=${classMap({
                        tab: true,
                        tabSelected: target === section,
                    })}
                    @click=${() => this.gotoRoute(target)}
                >
                    ${STR_SECTION_LABEL_LOOKUP[target]}
                </div>
                ${decorate ? html`<div>&gt;</div>` : nothing}
            `;
        };

        return html`
            <aside>
                <div class="left">
                    <div class="title">${STR_TITLE}</div>
                    <div class="tabs">
                        ${renderTab("general", true)}
                        ${renderTab("categories", true)}
                        ${renderTab("summary", false)}
                    </div>
                </div>
                <div class="right">
                    <button-rect-icon
                        @click=${() => this.gotoRoute("add")}
                        color="blue"
                        size="small"
                        iconBefore="plus"
                        >${STR_ADD}</button-rect-icon
                    >
                    <input-search .value=${query}></input-search>
                    <button-rect
                        @click=${() => this.gotoRoute("publish")}
                        color="orange"
                        size="small"
                        >${STR_PUBLISH}</button-rect
                    >
                </div>
            </aside>
        `;
    }
}
