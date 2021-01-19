import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
export type Color = "green" | "blue" | "white";
@customElement('icon-line')
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
icon: string = "";

@property()
label: string = "";

@property()
color: string = "";


  render() {
    const {icon, label,color} = this;
   

    return html`
<div class="wrapper">
        <img-ui path="${icon}"></img-ui>
        <p class="${color}">${label}</p>
</div>
  `;
  }
}