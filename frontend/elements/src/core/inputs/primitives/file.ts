import { LitElement, html, css, customElement, query, property } from "lit-element";

@customElement("input-file")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-auto-rows: minmax(0, 1fr);
                }
                label {
                    height: 100%;
                    width: 100%;
                    cursor: pointer;
                    display: grid;
                    place-content: center;
                }
                input {
                    display: none;
                }
            `,
        ];
    }

    @property()
    accept: string = "";

    @query("input")
    input!: HTMLInputElement;

    private onChange(e: any) {
        if (e.target.files[0]) {
            this.newFile(e.target.files[0]);
            // clear input so that selecting the file again will trigger onchange
            e.target.value = "";
        }
    }

    private onDrop(e: DragEvent) {
        e.preventDefault();
        if (e.dataTransfer?.files[0]) {
            this.newFile(e.dataTransfer.files[0]);
        }
    }

    private onDragOver(e: DragEvent) {
        e.preventDefault();
    }

    private newFile(file: File) {
        this.dispatchEvent(
            new CustomEvent("custom-file", {
                detail: file,
            })
        );
    }

    render() {
        return html`
            <label @drop="${this.onDrop}" @dragover="${this.onDragOver}" part="wrapper">
                <slot></slot>
                <input type="file" @change="${this.onChange}" accept=${this.accept} />
            </label>
        `;
    }
}
