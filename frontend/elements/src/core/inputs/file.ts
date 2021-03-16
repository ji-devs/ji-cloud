import { LitElement, html, css, customElement, query } from "lit-element";


@customElement("input-file")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                }
                label {
                    height: 100%;
                    width: 100%;
                    cursor: pointer;
                }
                input {
                    display: none;
                }
            `,
        ];
    }

    @query("input")
    input!: HTMLInputElement;

    private onChange(e: any) {
        if(e.target.files[0]) {
            this.newFile(e.target.files[0]);
        }
    }

    private onDrop(e: DragEvent) {
        e.preventDefault();
        if(e.dataTransfer?.files[0]) {
            this.newFile(e.dataTransfer.files[0]);
        }
    }

    private onDragOver(e: DragEvent) {
        e.preventDefault();
    }

    private newFile(file: File) {
        console.log(file);
        this.dispatchEvent(new CustomEvent("custom-file-change", {
            detail: { file },
        }))
    }

    render() {
        return html`
            <label @drop="${this.onDrop}" @dragover="${this.onDragOver}">
                <slot></slot>
                <input type="file" @change="${this.onChange}">
            </label>
        `;
    }
}
