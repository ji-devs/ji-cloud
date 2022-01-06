import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("star-rating")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: repeat(3, 20px);
                }
                fa-icon {
                    cursor: pointer;
                }
                fa-icon[icon="fa-solid fa-star"] {
                    color: #ffa41c;
                }
            `,
        ];
    }

    @property({ type: Number })
    rating?: number;

    private onClick(num: number) {
        let rating: number | undefined = num;

        // TODO: enable once rating can be unset
        // if(num === this.rating) {
        //     rating = undefined;
        // }

        this.dispatchEvent(new CustomEvent("custom-rating-change", {
            detail: {
                rating,
            }
        }));
    }

    private icon(num: number) : string {
        let rating = this.rating || 0;
        if(rating >= num) {
            return "fa-solid fa-star"
        } else {
            return "fa-regular fa-star";
        }
    }

    render() {
        return html`
            <fa-icon @click=${() => this.onClick(1)} icon=${this.icon(1)}></fa-icon>
            <fa-icon @click=${() => this.onClick(2)} icon=${this.icon(2)}></fa-icon>
            <fa-icon @click=${() => this.onClick(3)} icon=${this.icon(3)}></fa-icon>
        `;
    }
}
