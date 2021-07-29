import { LitElement, html, css, customElement, property, unsafeCSS } from "lit-element";
import { mediaUi } from "@utils/path";
import {ThemeId} from "@elements/_themes/themes";


@customElement("memory-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`

            :host {
                position: absolute;
                top: 0;
                left: 0;
            }
                .bg {
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: var(--content-width);
                    height: var(--content-height);
                }
                .fg {
                    display: grid;
                    grid-template-columns: 1fr calc(1432rem * (1920/1719));
                    column-gap: calc(56rem * (1920/1719));
                    width: 100%;
                    height: 100%;
                    background-size: cover;
                    position: absolute;
                    top: 0;
                    left: 0;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="bg"><slot name="bg"></slot></div>
            <div class="fg">
                <slot name="sidebar"></slot>
                <div>
                    <slot name="header"></slot>
                    <slot name="main"></slot>
                </div>
            </div>
        `
    }
}
