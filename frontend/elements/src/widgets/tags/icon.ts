import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';

export type IconKind = "age" | "lang";

@customElement('tag-icon')
export class _ extends LitElement {

  static get styles() {
    return [css`
    p{
        font-size: 14px;
        font-weight: 500;
        color:#798b96;
        margin-left:8px;
  
    }
    .wrapper{
        display:flex;
        align-items:center;
    }
    .darkgrey {
        color:#798b96;
    }
    .lightblue{
        color:#afcbf4;
    }
    `];
  }

    @property()
    kind: IconKind = "age";

    @property()
    label: string = "";

  render() {
    const {kind, label} = this;
   
    const iconPath = kind === "age" ? "Icn_Age.svg"
        : kind === "lang" ? "icn-language.svg"
        : "";

    const colorClass = kind === "age" ? "darkgrey"
        : kind === "lang" ? "lightblue"
        : "";

    return html`
<div class="wrapper">
        <img-ui path="${iconPath}"></img-ui>
        <p class="${colorClass}">${label}</p>
</div>
  `;
  }
}