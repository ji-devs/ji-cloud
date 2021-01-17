import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
export type Color = "green" | "blue" | "white";
@customElement('age-group')
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
    `];
  }

@property()
icon: string = "";

@property()
label: string = "";


  render() {
    const {icon, label} = this;
   

    return html`
<div class="wrapper">
        <img-ui path="${icon}"></img-ui>
        <p>${label}</p>
</div>
  `;
  }
}