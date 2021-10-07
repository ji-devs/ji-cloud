import { LitElement, html, css, customElement, property } from "lit-element";

const STR_HEADER = "Select theme";

@customElement("theme-selector")
export class _ extends LitElement {
  static get styles() {
      return [css`
          .options {
              margin-top: 93px;
              display: grid;
              grid-template-columns: repeat(2, 1fr);
              row-gap: 30px;
          }
          @media (min-width: 1920px) {
              .options {
                  row-gap: 47px;
              }
          }
          h2 {
              margin: 0px;
              font-family: Poppins;
              font-weight: normal;
              font-size: 16px;
          }
          @media (min-width: 1920px) {
              h2 {
                  font-size: 18px;
              }
          }
    `];
  }

  render() {

      return html`
          <h2>${STR_HEADER}</h2>
          <div class="options">
              <slot></slot>
          </div>
      `
  }
}
