import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
import "@elements/core/images/ji";

@customElement("profile-image")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                img-ui,
                img-ji {
                    display: inline-block;
                    height: inherit;
                    width: inherit;
                    border-radius: 50%;
                    overflow: hidden;
                }
            `,
        ];
    }

    @property()
    imageId?: string;

    render() {
        return html`
            ${this.imageId === undefined
                ? html`
                      <img-ui
                          path="user/profile-image-placeholder.webp"
                      ></img-ui>
                  `
                : html`
                      <img-ji
                          lib="user"
                          size="thumb"
                          id="${this.imageId}"
                      ></img-ji>
                  `}
        `;
    }
}
