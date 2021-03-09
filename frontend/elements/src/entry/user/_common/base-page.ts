import { mediaUi } from "@utils/path";
import { LitElement, html, css, customElement, property, unsafeCSS } from "lit-element";

const jigglingImage = mediaUi('/entry/user/jigglings.png');

@customElement("base-page")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        section {
            display: grid;
            grid-template-columns: 480px 1fr;
        }
        aside {
          min-height: 100vh;
          background-color: #def4ff;
          background-image: url("${unsafeCSS(jigglingImage)}");
          background-repeat: no-repeat;
          background-attachment: inherit;
          background-position: center;
        }

        article {
            padding: 80px;
            display: flex;
            flex-direction: column;
            gap: 20px;
        }
      `,
    ];
  }

  render() {

      return html`
          <section>
              <aside></aside>
              <article>
                  <slot></slot>
              </article>
          </section>
          `;
  }
}
