import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {MODE} from "@elements/module/memory/_common/types.ts";

const STR_DUPLICATE = "Duplicate";
const STR_WORDS_IMAGES = "Words & Images";
const STR_BEGINS = "What begins with...";
const STR_LETTERING = "Lettering";

@customElement('choose-card')
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
              <img-ui class="image" path="module/memory/choose/${mode}.png" alt="${label}"></img-ui>
          </section>
      `
  }
}
