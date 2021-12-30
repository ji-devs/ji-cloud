import { LitElement, html, css, customElement, property } from "lit-element";
import jigFlex from './jig-flex-css';
import "@elements/core/overlays/dialog-overlay";
import "@elements/entry/admin/curation/jig-details";

@customElement("admin-curation-single-jig")
export class _ extends LitElement {
    static styles = [
        jigFlex,
        css`
            .jig {
                display: flex;
                justify-content: space-between;
            }

            #overlay-container {
                width: 100%;
            }

            a {
                cursor: pointer;
            }

            ::slotted([name="jig-name"]) a {
                color: red;
            }

            dialog-overlay {
                background-color: #00000080;
            }
        `,
    ];

    render() {
        return html`
            <div class="jig">
                <div class="flex">
                    <a>
                        <slot name="jig-name"></slot>
                    </a>
                </div>
                <div class="flex">
                    <slot name="author"></slot>
                </div>
                <div class="flex">
                    <slot name="author-badge"></slot>
                </div>
                <div class="flex">
                    <slot name="date"></slot>
                </div>
                <div class="flex">
                    <slot name="language"></slot>
                </div>
                <div class="flex">
                    <slot name="curators"></slot>
                </div>
                <div class="flex">
                    <slot name="age-ranges"></slot>
                </div>
                <div class="flex">
                    <slot name="affiliations"></slot>
                </div>
            </div>
        `;
    }
}
