import { LitElement, html, css, customElement, property } from "lit-element";

export type IconKind = "record" | "success" | "upload";

@customElement("audio-input-icon")
export class _ extends LitElement {
    static get styles() {
        return [css``];
    }

    @property({ type: String, reflect: true })
    kind: IconKind = "record";

    render() {
        return html`
            <img-ui
                path="module/_common/edit/widgets/sidebar/audio-input/icon-${this
                    .kind}.svg"
            ></img-ui>
        `;
    }
}
