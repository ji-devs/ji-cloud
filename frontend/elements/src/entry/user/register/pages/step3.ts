import { mediaUi } from "@utils/path";
import { LitElement, html, css, customElement, property, unsafeCSS } from "lit-element";
import "@elements/core/lists/list-horizontal";
import "@elements/core/lists/list-vertical";
import "@elements/core/titles/ji";
import "@elements/entry/user/_common/base-page";

const STR_TITLE = "Sign Up - Step 3";
const STR_SUBTITLE = "We want to tailor the content that you find to your interests and needs.";
const STR_SUBSUBTITLE = "You can select as many as you like now and edit it later it in your profile page";
const STR_AGES_LABEL = "Which age group are you interested in?";
const STR_AFFILIATIONS_LABEL = "Content from which streams of Judaism do you want to see?";

@customElement("page-register-step3")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        .card-grey {
            padding: 32px 32px 32px 32px;
            border-radius: 14px;
            background-color: #f7f7f7;
        }
        h1 {
          font-size: 32px;
          font-weight: 900;
          color: #5662a3;
        }

        .subtitle {
          white-space: nowrap;
        }

        .submit {
            align-self: flex-end;
        }
      `,
    ];
  }

  render() {

      return html`
          <base-page>
                  <h1>${STR_TITLE}</h1>

                  <title-ji class="subtitle" size="subMedium">${STR_SUBTITLE}<br/>${STR_SUBSUBTITLE}</title-ji>
                  <div class="card-grey">
                      <list-horizontal label="${STR_AGES_LABEL}">
                          <slot name="ages"></slot>
                      </list-horizontal>
                  </div>
                  <div class="card-grey">
                      <list-vertical label="${STR_AFFILIATIONS_LABEL}">
                          <slot name="affiliations"></slot>
                      </list-vertical>
                  </div>
                  <div class="submit">
                      <slot name="submit"></slot>
                  </div>
          </base-page>
          `;
  }
}
