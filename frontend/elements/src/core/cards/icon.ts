import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";

export type IconKind = "group";

@customElement("card-icon")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        main{
            width: 122px;
            display:flex;
            flex-direction:column;
            justify-content:center;
            align-items:center;
        }
        .banner{
            display:flex;
            justify-content:center;
            align-items: center;
            font-weight:600;
            color:#272727;
            background-color:#d8e7fa;
            height: 32px;
            width: 122px;
            border-radius:0 0 12px 12px;
        }
    `,
    ];
  }

  @property()
  label: string = "";

  @property()
  icon: IconKind = "group";

  render() {
    const { label, icon } = this;

    const src = "core/cards/icon-group.svg"; 

    return html`
      <main>
        <img src="${MEDIA_UI}/${src}" />
        <div class="banner">${label}</div>
      </main>
        
    `;
  }
}
