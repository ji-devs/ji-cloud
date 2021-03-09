import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

export type MODE = "duplicate" | "words-images" | "begins" | "lettering";

const STR_DUPLICATE = "Duplicate";
const STR_WORDS_IMAGES = "Words & Images";
const STR_BEGINS = "What begins with...";
const STR_LETTERING = "Lettering";

@customElement('choose-card')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              display: block;
          width: 389px;
          height: 387px;
          border-radius: 24px;
          background-color: #c8defd;
          cursor: pointer;
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
          color: var(--Dark_Gray_6);
        }
    `];
  }

  @property()
  mode:MODE = "duplicate"

  render() {
      const {mode} = this;

      const label = mode === "duplicate" ? STR_DUPLICATE
        : mode === "words-images" ? STR_WORDS_IMAGES
        : mode === "begins" ? STR_BEGINS
        : mode === "lettering" ? STR_LETTERING
            : "";

      return html`
          <section>
            <div class="label">${label}</div>  
          </section>
      `
  }
}
