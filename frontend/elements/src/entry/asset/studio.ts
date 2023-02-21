import { LitElement, html, css, customElement, property, unsafeCSS, state } from "lit-element";
import "@elements/core/images/ui";
import { mediaUi } from "@utils/path";

@customElement("asset-edit-studio")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    min-height: 100dvh;
                    grid-template-rows: auto 1fr;
                    background-image: url(${unsafeCSS(
                        mediaUi("entry/jig/gallery/background.png")
                    )});
                    background-color: var(--light-blue-6);
                    background-size: 100%;
                }
                main {
                    max-width: 1600px;
                    width: 100vw;
                    padding: 48px 40px;
                    display: grid;
                    grid-template-rows: auto 1fr auto;
                    row-gap: 48px;
                    justify-content: center;
                    box-sizing: border-box;
                    margin: 0 auto;
                }
                h1 {
                    font-size: 28px;
                    font-weight: 900;
                    color: var(--main-yellow);
                    margin: 0;
                }
                h3 {
                    font-size: 18px;
                    font-weight: normal;
                    color: #fff;
                    margin: 0;
                }
                .gallery-links {
                    display: flex;
                    flex-wrap: wrap;
                    column-gap: 32px;
                }
                .gallery-link {
                    width: 212px;
                    height: 304px;
                    display: grid;
                    grid-template-rows: 1fr auto auto;
                    justify-items: center;
                    row-gap: 16px;
                    background-color: #ffffff;
                    box-shadow: 0 3px 10px 0 rgba(0, 0, 0, 0.16);
                    padding: 16px 8px;
                    box-sizing: border-box;
                    border-radius: 16px;
                }
                .gallery-link img-ui {
                    max-width: 100%;
                    max-height: 180px;
                }
            `,
        ];
    }

    render() {
        return html`
            <slot name="header"></slot>

            <main>
                <div>
                    <h1>Welcome to Jigzi Studio!</h1>
                    <h3>What do you want to create today?</h3>
                </div>

                <div class="gallery-links">
                    <div class="gallery-link">
                        <img-ui path="entry/jig/gallery/jig-icon.webp"></img-ui>
                        <slot name="jig-create"></slot>
                        <slot name="jig-gallery"></slot>
                    </div>
                    <div class="gallery-link">
                        <img-ui path="entry/jig/gallery/course-icon.webp"></img-ui>
                        <slot name="course-create"></slot>
                        <slot name="course-gallery"></slot>
                    </div>
                    <div class="gallery-link">
                        <img-ui path="entry/jig/gallery/resource-icon.webp"></img-ui>
                        <slot name="resource-create"></slot>
                        <slot name="resource-gallery"></slot>
                    </div>
                    <!-- <div class="gallery-link">
                        <img-ui path="entry/jig/gallery/pro-dev-icon.webp"></img-ui>
                        <slot name="pro-dev-create"></slot>
                        <slot name="pro-dev-gallery"></slot>
                    </div> -->
                </div>

                <h3><strong>Need help?</strong> Visit our help page <slot name="help"></slot></h3>
            </main>
        `;
    }
}
