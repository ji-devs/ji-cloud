import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import { nothing} from 'lit-html';
import {BaseButton} from "@elements/_styles/buttons";

export type Color = "red" | "blue" | "white" | "green" | "whiteblue";
export type Size = "small" | "medium" | "large" | "x-large";

export type IconAfter = "arrow";
export type IconBefore = "magnifier" | "share" | "create" | "play" | "plus";


@customElement("button-rect")
export class _ extends BaseButton {
  static get styles() {
    return [
      css`
        button {
          border-radius: 24px;
          border: none;
          cursor: pointer;
          font-size: 16px;
          display: flex;
          align-items: center;
        }
        .medium {
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
      .x-large{
        font-size:24px;
        padding: 13.6px 24px 11.4px;
      }
      .whiteblue{
        background-color:#ffffff;
        color:#5590fc;
      }
    
    `];
  }

  @property()
  size: Size = "medium";

  @property()
  color: Color = "red";

  @property({ type: Boolean })
  bold: boolean = false;

  @property({ type: Boolean })
  italic: boolean = false;

 

  @property()
  iconBefore?: IconBefore;
  @property()
  iconAfter?: IconAfter;

  render() {
    

    const {size, color, bold, italic,  iconAfter, iconBefore} = this;

    const classes = classMap({ 
      [size]: true,
      [color]: true,
      bold: bold,
      italic: italic,
     
    });

    
    const iconBeforePath = iconBefore === "magnifier" ? "core/inputs/magnifier.svg" 
    : iconBefore === "share" ? "core/inputs/share-red.svg" 
    : iconBefore === "create" ? "core/inputs/plus-red.svg" 
    : iconBefore === "play" ? "core/inputs/TODO-PLAY.svg"
    : iconBefore === "plus" ? getPlus(color)
    : iconBefore === "blueplay" ? "core/inputs/play-blue.svg"
    : nothing;
    const iconAfterPath = iconAfter === "arrow" ? "core/inputs/arrow.svg"
      : "";

    return html`
      <button type="button" name="button" class="${classes}" >
      ${iconBefore && html`<img-ui class="left" path="${iconBeforePath}"></img-ui>`}
      <slot class="${classes}"></slot>
      ${iconAfter && html`<img-ui class="right" path="${iconAfterPath}"></img-ui>`}
    </button>
  `;
  }
}

function getPlus(color:Color) {
  return color === "blue" ? "core/inputs/plus-white.svg"
    : nothing;
}