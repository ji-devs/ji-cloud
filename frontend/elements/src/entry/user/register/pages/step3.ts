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
const STR_SUBJECTS_LABEL = "Which subjects do you teach?";

@customElement("page-register-step3")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
      .grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 1fr 1fr;
        gap: 32px 31px;
        grid-auto-flow: row;
        grid-template-areas:
          "tl tr"
          "bottom bottom";
      }

      .ages { grid-area: tl; }
      .subjects { grid-area: tr; }
      .affiliations { grid-area: bottom; }

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
                  <div class="grid">
                    <div class="ages card-grey">
                        <list-vertical label="${STR_AGES_LABEL}">
                            <slot name="ages"></slot>
                        </list-vertical>
                    </div>
                    <div class="subjects card-grey">
                        <list-vertical label="${STR_SUBJECTS_LABEL}">
                            <slot name="subjects"></slot>
                        </list-vertical>
                    </div>
                    <div class="affiliations card-grey">
                        <list-vertical label="${STR_AFFILIATIONS_LABEL}">
                            <slot name="affiliations"></slot>
                        </list-vertical>
                    </div>
                  </div>
                  <div class="submit">
                      <slot name="submit"></slot>
                  </div>
          </base-page>
          `;
  }
}
