import { LitElement, html, css, customElement } from "lit-element";

const STR_DELETE = 'Delete';

@customElement("audio-input-delete")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .content {
                    display: flex;
                    align-items: center;
                    column-gap: 3px;
                }
            `,
        ];
    }

    render() {
        return html`
            <button-rect kind="text" color="blue">
                <div class="content">
                    <img-ui path="module/_common/edit/widgets/sidebar/audio-input/delete.svg"></img-ui>
                    ${ STR_DELETE }
                </div>
            </button-rect>
        `;
    }
}
