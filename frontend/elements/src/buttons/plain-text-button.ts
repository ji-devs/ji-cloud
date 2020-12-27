import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import {BaseButton} from "@elements/_styles/buttons";



@customElement('plain-button')
export class _ extends BaseButton {

  static get styles() {
    return [css`
      p{
        font-weight: 500;
        cursor: pointer;
      }
      .small{
        padding: 0;
      }
      .medium{
          padding: 13.6px 24px 11.4px;
      }
      .large{
       padding: 15px 40px 16px;
      }
      
      .red {
        color:#fd6b71;
      }
      
      .red:hover{
        color: #ed6065
      }
      
      .blue{
        color:#5590fc;
      }
      
      .blue:hover{
        color: #387af4
      }
      
      button:disabled{
        color: #a9b1b5;
        
      }
  
      .bold {
        font-weight: bold;
      }
      .green{
        color: #71cf92;
        
      }
      .green:hover{
        color: #46ba6f;
      }
  
    
    `];
  }

  @property()
  size: string = "";

  @property()
  label: string = "";

  @property()
  color: "red" | "blue" | "" = "";

  @property({type: Boolean})
  bold:boolean = false; 

  @property({type: Boolean})
  italic:boolean = false; 

  render() {

    const {size, label, color, bold, italic} = this;

    const classes = classMap({ 
      [size]: true,
      [color]: true,
      bold: bold,
      italic: italic,
    });

    return html`
      <p  class="${classes}" >
      ${label}
  </p>
  `;
  }
}