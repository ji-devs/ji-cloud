import {
    LitElement,
    svg,
    html,
    css,
    customElement,
    property,
} from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

export type Color = "blue";

@customElement("box-outline-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                  --border-size: 1px;
                }

                :host([thick]) {
                  --border-size: 5px;
                }

                
                section {
                    box-sizing: border-box;
                    cursor: pointer;
                    position: relative;
                    left: calc(var(--border-size) * -1);
                    display: inline-block;
                    box-shadow: 0 0 6px 0 rgba(0, 0, 0, 0.16);
                }

                :host([borderInside]) > section {
                  left: 0;
                }

                :host([color="blue"]) > section {
                    border: solid var(--border-size) var(--main-blue);
                }

                :host([hover]) .close {
                  display: block;
                }

                .close {
                  display: none;
                  position: absolute;
                  top: calc((12px + var(--border-size)) * -1); 
                  right: calc((12px + var(--border-size)) * -1); 
                  width: 24px;
                  height: 24px;
                }

                .audio {
                  top: calc((32px + var(--border-size) + 4px) * -1);
                  left: calc(var(--border-size) * -1);
                  position: absolute;
                  width: 32px;
                  height: 32px;
                }
            `,
        ];
    }

    @property({ reflect: true })
    color: Color = "blue";

    @property({type: Boolean, reflect: true})
    thick:boolean = false;

    @property({type: Boolean, reflect: true})
    uncloseable:boolean = false;

    @property({type: Boolean, reflect: true})
    "borderInside":boolean = false;

    @property({type: Boolean, reflect: true})
    hover:boolean = false;

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

    onClose() {
      this.dispatchEvent(new Event("close"));
    }

    render() {
      const {uncloseable} = this;

        return html`
            <section>
                <slot></slot>
                <div class="audio">
                  <slot name="audio">
                    <!--<button-icon icon="audio"></button-icon>-->
                  </slot>
                </div> 
                ${uncloseable 
                  ? nothing
                  : html`<button-icon class="close" icon="circle-x-blue" @click=${this.onClose}></button-icon>`
                }
            </section>
        `;
    }
}
