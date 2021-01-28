import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
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
        .yellow{

         }
        .pink{

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

    

    return html`

    <img-ui path="chevron_arrow_yellow_right.svg" class="${direction}" ></img-ui>
    <img-ui path="chevron_arrow_yellow_right.svg" class="${direction}" ></img-ui>

    // class="${direction} ${color}"
  
    `;
  }
}
  