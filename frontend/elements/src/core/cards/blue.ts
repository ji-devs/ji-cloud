import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("card-blue")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        div{
            height: 650px;
            border-radius: 10px;
            background-color: #edf2ff;
            overflow:auto;
            width:600px;
            padding:32px 40px;
        }
        ::slotted([slot=content]:nth-child(even)){
            margin-bottom: 24px;
        }
  
    `,
    ];
  }

  @property()
  label: string = "";

  render() {
    const { label } = this;
    return html`
      <div>
        <slot></slot>
      </div>
        
    `;
  }
}
