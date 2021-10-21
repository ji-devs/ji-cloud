import { LitElement, html, css, customElement } from "lit-element";
import "@elements/core/images/ui";
import "@elements/entry/jig/_common/bg";

@customElement("jig-edit-page")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                section {
                    display: flex;
                    height: 100%;
                }
                .sidebar {
                    z-index: 1;
                }
                .main {
                    width: 100%;
                }
            `,
        ];
    }

    render() {
        return html`
            <bg-jig>
                <section>
                    <div class="sidebar"><slot name="sidebar"></slot></div>
                    <div class="main"><slot name="main"></slot></div>
                </section>
            </bg-jig>
        `;
    }
}
