import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import { styleMap } from "lit-html/directives/style-map";

@customElement("legacy-input-fit")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                }
            `,
        ];
    }

    firstUpdated(_changed: any) {
        this.resize();
    }
    updated(_changed: any) {
        this.resize();
    }

    onInput() {
        this.resize();

        const input = this.shadowRoot?.getElementById("input") as HTMLInputElement;
        this.dispatchEvent(
            new CustomEvent("custom-input", {
                detail: { value: input.value },
            })
        );
    }

    onKey(evt: KeyboardEvent) {
        let { key } = evt;
        key = key.toLowerCase();
        if (key === "enter") {
            this.dispatchEvent(new Event("enter"))
        }
    }

    resize() {
        const input = this.shadowRoot?.getElementById("input") as HTMLInputElement;

        const isOverflowing = () => {
            return input.clientWidth < input.scrollWidth || input.clientHeight < input.scrollHeight;
        }

        let curr = 12;
        const max = 128;

        do {
            input.style.fontSize = `${curr++}px`;
        } while(!isOverflowing() && curr < max);

        input.style.fontSize = `${curr - 1}px`;

        console.log(curr);
        
    }

    @property({type: Number})
    x:number = 0;

    @property({type: Number})
    y:number = 0;

    @property({type: Number})
    width:number = 0;

    @property({type: Number})
    height:number = 0;

    @property()
    value:string = "";

    render() {
        
        const {x, y, width, height, value} = this;

        const style = styleMap({
            position: "absolute",
            top: `${y}px`,
            left: `${x}px`,
            width: `${width}px`,
            height: `${height}px`,
        });
        return html`
            <input 
                id="input"
                type="text"
                autocomplete="off"
                autocorect="off"
                autocapitalize="none"
                spellcheck="false"
                style=${style}
                @input="${this.onInput}"
                @keyup="${this.onKey}"
                .value="${value}"
            ></input>
        `;
    }
}
