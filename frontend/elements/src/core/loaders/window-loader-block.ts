import { mediaUi } from "@utils/path";
import { LitElement, html, css, customElement, property, unsafeCSS } from "lit-element";
import { nothing } from "lit-html";

@customElement("window-loader-block")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                section {
                    position: fixed;
                    top: 0;
                    left: 0;
                    width: 100vw;
                    height: 100vh;
                    background-color: #6ca1fc3d;
                    background-image: url(${unsafeCSS(
                        mediaUi("core/loaders/large.svg")
                    )});
                    background-position: center center;
                    background-repeat: no-repeat;
                    z-index: 1000;
                }
            `,
        ];
    }

    @property({ type: Boolean })
    visible: boolean = false;

    render() {
        const { visible } = this;
        return visible ? html`<section></section>` : nothing;
    }
}
