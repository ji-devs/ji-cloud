import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";


@customElement('choose-mode-option')
export class _ extends LitElement {
  static get styles() {
      return [css`
        section {
          display: flex;
          width: 389px;
          height: 387px;
          border-radius: 24px;
          background-color: #c8defd;
          cursor: pointer;
          flex-direction: column;
          justify-content: center;
          align-items: center;
        }

        section:hover {
            background-color: #bed8ff;
        }


        .label {
            margin-top: 64px;
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
  render() {
      const {module, mode, label} = this;

      return html`
          <section>
              <div class="label">${label}</div>  
              <img-ui class="image" path="module/${module}/edit/choose/${mode}.png" alt="${label}"></img-ui>
          </section>
      `
  }
}
