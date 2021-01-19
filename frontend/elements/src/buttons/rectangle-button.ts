import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import {BaseButton} from "@elements/_styles/buttons";

export type Color = "red" | "blue" | "white" | "green";
export type Size = "small" | "medium" | "large";

@customElement('rectangle-button')
export class _ extends BaseButton {

  static get styles() {
    return [css`
    button {
        border-radius: 24px;
        border: none;
        cursor: pointer;
        font-size: 16px;
        display:flex;
        align-items:center;
        
        
      }
      .medium{
          padding: 13.6px 24px 11.4px;
      }
      .large{
       padding: 15px 40px 16px;
      }
      
      .red {
        background:#fd6b71;
        color:#fff;
      }
      
      .red:hover{
        background: #ed6065
      }
      
      .blue{
        background:#5590fc;
        color:#ffffff;
      }
      
      .blue:hover{
        background: #387af4
      }
      
      button:disabled{
        background: #a9b1b5;
        color: #e7edf0
      }
      button:focus{
          outline:none;
      }
      .bold {
        font-weight: bold;
      }
      .green{
        background-color: #71cf92;
        color:#ffffff;
      }
      .green:hover{
        background-color: #46ba6f;
      }
      .white{
        border: solid 1px #fb6c74;
        color:#fb6c74;
        background: #ffffff;
      }
      .left{
        margin-right:8px;
      }
      .pink{
        background-color:#fd6b71;
        color:#fffde6;
      }
      .right 
      {
        margin-left:8px;
      }
      .img-hidden{
        display:none
      }
    
    `];
  }

  @property()
  size: Size = "medium";

  @property()
  label: string = "";

  @property()
  color: Color = "red";

  @property({type: Boolean})
  bold:boolean = false; 

  @property({type: Boolean})
  italic:boolean = false; 

  @property({type: Boolean})
  imglefthidden:boolean = false; 

  @property({type: Boolean})
  imgrighthidden:boolean = false; 

  @property()
  iconpath: string = "";

  render() {

    const {size, label, color, bold, italic, imglefthidden, imgrighthidden,iconpath} = this;

    const classes = classMap({ 
      [size]: true,
      [color]: true,
      bold: bold,
      italic: italic,
    });

    return html`
      <button type="button" name="button" class="${classes}" >
      <img-ui class="${imglefthidden ? 'img-hidden' : ''} left" path="${iconpath}"></img-ui>
      ${label}
      <img-ui class="${imgrighthidden ? 'img-hidden' : ''} right" path="${iconpath}"></img-ui>
    </button>
  `;
  }
}