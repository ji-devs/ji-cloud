import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import "@elements/core/inputs/text-pencil";
import "@elements/core/buttons/icon";
import "@elements/core/buttons/text";


@customElement("jig-edit-sidebar")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
      :host {
          display: flex;
          flex-flow: column;
          height: 100vh;
            width: 416px;
            box-shadow: 0 3px 20px 0 rgba(0, 0, 0, 0.08);
            background-color: var(--white);
      }


      header {
          margin-left: 24px;
          margin-top: 8px;
            width: 384px;
      }

      section {
          margin-top: 7px;
          height: 100%;
          overflow-y: auto;
          overflow-x: hidden;
      }

      .cover-module {
          /* Allow room for jiggling head */
          margin-top: 200px;
      }
    
      .modules {
          /* Allow room for jiggling feet */
          margin-bottom: 100px;
      }

      `,
    ];
  }

  render() {

    return html`
        <header>
            <slot name="header"></slot>
        </header>
        <section>
            <div class="cover-module">
                <slot name="cover-module"></slot>
            </div>
            <div class="modules">
                <slot name="modules"></slot>
            </div>
        </section>

    `;
  }
}
