import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/buttons/button-collection/button-collection";
import "@elements/core/buttons/button-collection/button-collection-item";


@customElement("input-inc-dec")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            `,
        ];
    }

    @property({type: Number})
    value: number = 1;

    @property({type: Number})
    min: number = 0;

    @property({type: Number})
    max: number = 10;

    private changed() {
        this.dispatchEvent(new CustomEvent("custom-change", {
            detail: { value: this.value },
        }))
    }

    private increment() {
        if (this.value < this.max) {
            this.value++;
            this.changed();
        }
    }

    private decrement() {
        if (this.value > this.min) {
            this.value--;
            this.changed();
        }
    }

    render() {
        return html`
            <button-collection>
                <button-collection-item @click="${this.increment}">+</button-collection-item>
                <button-collection-item>${ this.value }</button-collection-item>
                <button-collection-item @click="${this.decrement}">-</button-collection-item>
            </button-collection>
        `;
    }
}
