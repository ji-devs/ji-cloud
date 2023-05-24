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
                    border-radius: var(--border-radius, 0);
                }
            `,
        ];
    }

    @property()
    videoId?: string;

    //use with cacheBust true to force reloading when id changes to the same thing
    @property()
    borderRadius: string = "0";

    firstUpdated() {
        this.style.setProperty('--border-radius', this.borderRadius);
    }

    updated(changedProperties: Map<string | number | symbol, unknown>) {
        if (changedProperties.has('borderRadius')) {
            this.style.setProperty('--border-radius', this.borderRadius);
        }
    }

    render() {
        return html`
            <img
                src="https://i.ytimg.com/vi_webp/${this.videoId}/mqdefault.webp"
            />
        `;
    }
}
