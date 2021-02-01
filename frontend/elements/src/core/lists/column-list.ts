import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';

export type Color="white"|"black"|"darkBlue";
export type Size="medium"|"normal";

@customElement('column-list')
export class _ extends LitElement {
    static get styles() {
    return [css`
    .normal li{
      font-family: Poppins;
      font-size: 14px;
      font-weight: 200;
      color:#ffffff;
 
     }

     li{
      list-style-type: none;

     }
     .bold {
       font-weight:600;
       color:#ffffff;

     }



     .medium{
      font-size: 20px;
      font-weight: normal;
 
     }

     .white{
       color:#ffffff;
     }

     .black{
       color:#383838;
     }

     .darkBlue{
     color=#192150;
     }
    `];
  }
  @property()
  text_line:string = ""; 
  @property({type:Boolean})
  bold:boolean = false; 

  @property()
  size:Size = "normal"; 
  @property()
  color:Color = "black"; 
 

  render() {
    const {text_line, bold,size,color} = this;

    return html`
     <li class="${bold } ${ size } ${color}">${text_line}</li>
    
  `;
  }
}