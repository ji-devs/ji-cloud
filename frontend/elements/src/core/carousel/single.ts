import { LitElement, html, css, customElement, property, query } from 'lit-element';

@customElement('carousel-single')
export class CarouselSingle extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: block;
            }
            .carousel {
                display: grid;
                grid-auto-flow: column;
                overflow-x: auto;
                grid-auto-columns: 100%;
                scroll-snap-type: x mandatory;
                scrollbar-width: none; /* Firefox */
            }
            .carousel::-webkit-scrollbar {
                display: none; /* Chromium and Safari */
            }
            .carousel ::slotted(*) {
                scroll-snap-align: center;
                scroll-snap-stop: always;
            };
        `];
    }

    @query(".carousel")
    private carousel!: HTMLElement;

    public back() {
        this.goToPage(this.activePage - 1);
    }

    public forward() {
        this.goToPage(this.activePage + 1);
    }

    private getActivePage() : number {
        return Math.round(this.carousel.scrollLeft / this.clientWidth);
    }

    @property({ type: Number })
    private activePage: number = 0;

    private onScroll() {
        this.activePage = this.getActivePage();
        this.dispatchEvent(new CustomEvent("active-page-change", {
            detail: {activePage: this.activePage}
        }));
    }

    public goToPage(page: number) {
        this.carousel.scroll({
            left: page * this.clientWidth,
            behavior: 'smooth',
        });
    }

    render() {
        return html`
            <div class="carousel" @scroll="${this.onScroll}">
                <slot></slot>
            </div>
        `;
    }
}
