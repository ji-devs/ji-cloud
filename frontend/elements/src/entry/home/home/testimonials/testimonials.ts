import { LitElement, html, css, customElement, property } from "lit-element";
import { homeStyles } from "../styles";
import "./testimonial-carousel";

const STR_START_TITLE = "What they say ";
const STR_ABOUT_US = "about us ";

@customElement("home-testimonials")
export class _ extends LitElement {
    static get styles() {
        return [
            homeStyles,
            css`
                :host {
                    background-color: var(--main-red);
                    display: block;
                    padding: 72px 0;
                }
                .width-holder {
                    display: grid;
                    grid-template-rows: auto 165px auto;
                }
                h2 {
                    margin: 0;
                    text-align: center;
                    color: white;
                    font-size: 64px;
                    font-weight: 900;
                }
                h2 .about-us {
                    color: var(--main-yellow);
                }
                .face {
                    z-index: 1;
                    margin-top: -60px;
                }
                .carousels {
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    column-gap: 90px;
                    justify-content: space-between;
                }
                .carousel {
                    min-height: 380px;
                    display: inline-grid;
                }
                .carousel home-testimonial-carousel {
                    grid-column: 1;
                    grid-row: 1;
                    place-self: center;
                    z-index: 1;
                    padding: 10px 20px;
                }
                .carousel::after {
                    content: "";
                    grid-column: 1;
                    grid-row: 1;
                    height: 100%;
                    border-radius: 40px;
                    background-color: var(--light-red-4);
                }
                .carousel.teachers::after {
                    transform: rotate(5deg);
                }
                .carousel.parents::after {
                    transform: rotate(357deg);
                }
                .carousel.parents {
                    transform: translateY(-56px);
                }
            `,
        ];
    }

    @property({ type: Number })
    teachersPageCount = 1;

    @property({ type: Number })
    parentsPageCount = 1;

    render() {
        return html`
            <div class="width-holder">
                <h2>
                    ${STR_START_TITLE}
                    <span class="about-us">${STR_ABOUT_US}</span>
                </h2>
                <img-ui
                    class="face"
                    path="entry/home/testimonials/face.png"
                ></img-ui>
                <div class="carousels">
                    <div class="carousel teachers">
                        <home-testimonial-carousel
                            class="teachers-testimonials"
                            pageCount="${this.teachersPageCount}"
                        >
                            <slot name="teachers"></slot>
                        </home-testimonial-carousel>
                    </div>
                    <div class="carousel parents">
                        <home-testimonial-carousel
                            class="parents-testimonials"
                            pageCount="${this.parentsPageCount}"
                        >
                            <slot name="parents"></slot>
                        </home-testimonial-carousel>
                    </div>
                </div>
            </div>
        `;
    }
}
