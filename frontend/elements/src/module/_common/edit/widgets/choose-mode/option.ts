import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("choose-mode-option")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                section {
                    display: grid;
                    align-content: start;
                    justify-items: center;
                    border-radius: 24px;
                    background-color: #c8defd;
                    cursor: pointer;
                    box-sizing: border-box;
                    /* padding: 50px 0px 20px 0; */
                    width: 340px;
                    height: 340px;
                }
                @media (min-width: 1920px) {
                    section {
                        width: 388px;
                        height: 388px;
                    }
                }

                :host([hover]) section {
                    background-color: #bed8ff;
                }

                .label {
                    font-size: 24px;
                    font-weight: 300;
                    text-align: center;
                    color: var(--dark-gray-6);
                    margin-top: 50px;
                    margin-bottom: 10px;
                }
                @media (min-width: 1920px) {
                    .label {
                        margin-top: 64px;
                        margin-bottom: 32px;
                    }
                }

                .hidden {
                    display: none;
                }

                img-ui {
                    width: 340px;
                }
            `,
        ];
    }

    @property()
    module: string = "";

    @property()
    mode: string = "";

    @property()
    label: string = "";

    @property({ type: Boolean, reflect: true })
    hover: boolean = false;

    connectedCallback() {
        super.connectedCallback();

        this.addEventListener("mouseenter", this.onMouseEnter);
        this.addEventListener("mouseleave", this.onMouseLeave);
    }

    disconnectedCallback() {
        super.disconnectedCallback();

        this.removeEventListener("mouseenter", this.onMouseEnter);
        this.removeEventListener("mouseleave", this.onMouseLeave);
    }

    onMouseEnter() {
        this.hover = true;
    }

    onMouseLeave() {
        this.hover = false;
    }

    render() {
        const { module, mode, label, hover } = this;

        const imageClass = classMap({
            hidden: hover,
        });
        const imageHoverClass = classMap({
            hidden: !hover,
        });

        return html`
            <section>
                <div class="label">${label}</div>
                <img-ui
                    class=${imageClass}
                    path="module/${module}/edit/choose/${mode}.png"
                    alt="${label}"
                ></img-ui>
                <img-ui
                    class=${imageHoverClass}
                    path="module/${module}/edit/choose/${mode}-hover.png"
                    alt="${label}"
                ></img-ui>
            </section>
        `;
    }
}
