import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";


@customElement('choose-mode-option')
export class _ extends LitElement {
  static get styles() {
      return [css`
        section {
          display: flex;
          align-items: center;
          flex-direction: column;
          width: 389px;
          height: 387px;
          border-radius: 24px;
          background-color: #c8defd;
          cursor: pointer;
        }

        :host([hover]) section {
            background-color: #bed8ff;
        }


        .label {
            margin-top: 64px;
            margin-bottom: 32px;
          font-family: Poppins;
          font-size: 24px;
          font-weight: 300;
          font-stretch: normal;
          font-style: normal;
          letter-spacing: normal;
          text-align: center;
          color: var(--dark-gray-6);
        }
    `];
  }

  @property()
  module:String = "";

  @property()
  mode:String = "";

  @property()
  label:String = "";

  @property({type: Boolean, reflect: true})
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
      const {module, mode, label, hover} = this;

      const filename = hover ? `${mode}-hover.png` : `${mode}.png`;

      return html`
          <section>
              <div class="label">${label}</div>  
              <img-ui class="image" path="module/${module}/edit/choose/${filename}" alt="${label}"></img-ui>
          </section>
      `
  }
}
