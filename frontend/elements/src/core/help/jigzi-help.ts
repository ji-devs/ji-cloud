import { LitElement, html, css, customElement, property, query } from "lit-element";

@customElement("jigzi-help")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                #gear-img {
                    cursor: pointer;
                }
            `,
        ];
    }

    @property()
    title: string = "";

    @property()
    body: string = "";

    @property()
    showId: string = "";

    @query("#gear-img")
    imgRef: HTMLElement | undefined;

    //instead of firstUpdated since tooltip needs the size of the image to position correctly
    onImageLoaded() {
        this.requestUpdate();
    }

    render() {
        return html`
            <img-ui
                @image-load=${this.onImageLoaded}
                id="gear-img"
                path="module/_common/edit/header/jiggling-gear.png"
            ></img-ui>
            <slot></slot>
        `;
    }
}
