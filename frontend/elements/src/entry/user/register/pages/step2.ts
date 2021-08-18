import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/dividers/spacer-fourty";
import "@elements/entry/user/_common/base-page";

const STR_TITLE = "Sign Up - Step 2";

@customElement("page-register-step2")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        h1 {
          font-size: 32px;
          font-weight: 900;
          color: #5662a3;
        }

        .rows {
          display: flex;
          flex-direction: column;
          gap: 40px;
        }
        .row {
          display: grid;
          grid-template-columns: 1fr 1fr;
          gap: 32px;
        }

        .bottom {
          margin-top: 20px;
        }
      `,
    ];
  }

  render() {
      return html`
          <base-page>
          <h1>${STR_TITLE}</h1>
          <div class="rows">
            <div class="row">
              <slot name="location"></slot>
              <slot name="language"></slot>
            </div>
            <div class="row">
              <slot name="persona"></slot>
              <slot name="organization"></slot>
            </div>
          </div>
            <div class="bottom">
              <slot name="checkbox"> </slot>

              <p></p>
              <slot name="submit"></slot>
            </div>
      </base-page>
    `;
  }
}
