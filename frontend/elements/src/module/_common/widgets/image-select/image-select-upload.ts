import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/images/ui";


@customElement('image-select-upload')
export class _ extends LitElement {

    static get styles() {
        return [css`
            label {
                display: flex;
                align-items: center;
                column-gap: 4px;
                cursor: pointer;
                font-weight: 500;
                color: var(--main-blue);
            }
            input {
                display: none;
            }
        `];
    }

    @property()
    label: string = "";

    private onChange(e: any) {
        if(e.target.files[0]) {
            const file = e.target.files[0];
            console.log(file);
            this.dispatchEvent(new CustomEvent("custom-file-change", {
                detail: { file },
            }))
        }
    }

    render() {
        return html`
            <label>
                <img-ui path="module/_common/widgets/sidebar/image-select/upload-icon.svg"></img-ui>
                <span>${ this.label }</span>
                <input @input="${this.onChange}" type="file">
            </label>
        `;
    }
}
