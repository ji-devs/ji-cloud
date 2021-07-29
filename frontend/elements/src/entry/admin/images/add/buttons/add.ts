import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/buttons/rectangle-icon";

    const STR_UPLOAD = "Upload image";
@customElement('button-add')
export class _ extends LitElement {
  static get styles() {
      return [css`
          section {
                width: 288px;
              height: 216px;
              border-radius: 4px;
              background-color: #e6f0ff;
              display: flex;
              justify-content: center;
              align-items: center;
          }

          .inner-dash {
                width: 256px;
                height: 184px;
                border: dashed 3px #5590fc;
              display: flex;
              justify-content: center;
              align-items: center;
          }
    `];
  }

  render() {

      return html`
          <section>
              <div class="inner-dash">

                    <button-rect-icon color="blue" size="medium" iconBefore="plus">${STR_UPLOAD}</button-rect-icon>
              </div>
          </section>
    `;
  }
}
