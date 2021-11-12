import {
    LitElement,
    html,
    css,
    customElement,
    property,
    query,
} from "lit-element";
import "@elements/core/images/ui";

@customElement("video-youtube-thumbnail")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    width: 480px;
                    height: 270px;
                }
                img {
                    height: 100%;
                    width: 100%;
                }
            `,
        ];
    }

    @property()
    videoId?: string;

    render() {
        return html`
            <img
                src="https://i.ytimg.com/vi_webp/${this.videoId}/mqdefault.webp"
            />
        `;
    }
}
