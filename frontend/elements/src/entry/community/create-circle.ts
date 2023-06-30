import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";

const STR_CREATE = "Start a circle";

@customElement("community-create-circle")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                popup-body {
                    border-radius: 12px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    max-width: 620px;
                    width: 80vw;
                }
                .body {
                    padding: 0 24px 24px 24px;
                    display: grid;
                    grid-gap: 30px;
                    grid-template-areas: 
                        "name"
                        "description"
                        "image"
                        "submit";
                }
                @media (min-width: 1024px) {
                    .body {
                        grid-template-columns: 1fr 1fr;
                        grid-template-areas: 
                            "name        image"
                            "description image"
                            "submit      submit";
                    }
                }
                ::slotted([slot=close]) {
                    font-size: 14px;
                }
                ::slotted([slot=name]) {
                    grid-area: name;
                }
                ::slotted([slot=description]) {
                    grid-area: description;
                    height: 100px;
                }
                ::slotted([slot=image]) {
                    grid-area: image;
                    display: grid;
                    height: 188px;
                }
                ::slotted([slot=submit]) {
                    grid-area: submit;
                    justify-self: center;
                }
            `,
        ];
    }

    render() {
        return html`
            <popup-body>
                <slot slot="close" name="close"></slot>
                <h3 slot="heading">${STR_CREATE}</h3>
                <div slot="body" class="body">
                    <slot name="name"></slot>
                    <slot name="description"></slot>
                    <slot name="image"></slot>
                    <slot name="submit"></slot>
                </div>
            </popup-body>
        `;
    }
}
