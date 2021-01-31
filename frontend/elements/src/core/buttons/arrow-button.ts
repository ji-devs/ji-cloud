import { LitElement, html, css, customElement, property } from "lit-element";
 import { BaseButton } from "@elements/_styles/buttons";
import {nothing} from "lit-html";

export type Color = "yellow" | "pink"  ;
export type Direction = "right" | "left"  ;
 

@customElement("arrow-button")
export class _ extends BaseButton {
  static get styles() {
    return [
      css`

      button{
        width: 48px;
        height: 48px;
        cursor:pointer;
      }
  
 
 .right{

 }

 .left{
  transform: scaleX(-1);
display:block;
margin-left:100px;
 }


      `,
    ];
  }

  @property()
  direction:Direction   = "right";

  @property()
  color: Color  = "yellow";
  
  render() {
    const { direction , color } = this;
    const patharrow = color === "yellow" ? "chevron_arrow_yellow_right.svg"
    : color === "pink" ? "chevron_arrow_pink.svg"
     : nothing;
    

    return html`

    <img-ui  path="${patharrow}" class="${direction}" ></img-ui>
 
   
    `;
  }
}
  