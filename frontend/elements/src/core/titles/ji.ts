import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import {nothing} from "lit-html";
export type Color = "red" | "blue" | "white" | "green" | "lightgreen" | "lightblue" | "black" | "darkblue";
export type Size = "small" | "medium" | "title-medium" | "medium-large" | "large" | "title-large" | "x-large";
export type Weight = "lighter" | "light" | "normal" | "bold" | "bolder" | "boldest";

@customElement('title-ji')
export class _ extends LitElement {

  static get styles() {
    return [css`
    div{
        display:flex;
        align-items:center;
    }
   
    p{
        font-weight: 500;
        margin-right:36px;
    }
    
    .lighter{
      font-weight:300
    }
    .light{
      font-weight:400;
    }
    .normal{
      font-weight:500
    }
    .bold{
      font-weight:bold;
    }
    .bolder{
      font-weight:800;
    }
    .boldest{
      font-weight:900
    }

    .link {
      cursor: pointer;
    }

    .blue {
      color: #5590fc;
    }
    
    
    .black{
      color: #4a4a4a
    }
    .lightgreen{
      color: #9cddb2
    }
    .white{
      color:#ffffff
    }
    .x-large{
      font-size:64px;
    }
    .title-medium{
      font-size:24px;
    }
    .large{
      font-size:32px;
    }
    .medium-large{
      font-size:18px;
    }
    
    .lightblue{
      color:#e7f0fe;
    }
    .medium{
      font-size:16px;
    }
    
    .small{
      font-size:14px;
    }
    .title-large{
      font-size:40px;
    }
    .red{
      color:#fd7076;
    }
    .darkblue{
      color:#5662a3;
    }
    `];
  }


  @property({type: Boolean})
  italic:boolean = false;

  @property({type: Boolean})
  underlined:boolean = false;

  @property()
  size:Size = "medium";

  @property()
  color:Color = "red";

  @property()
  weight:Weight = "normal";

  @property({type: Boolean})
  p:boolean = false;

  @property({type: Boolean})
  link:boolean = false;

  render() {

    const {size, color, italic, weight, p, link} = this;

    const classes = classMap({ 
      [size]: true,
      [color]: true,
      italic: italic,
      [weight]:true,
      link,
    });
    return html`
    ${p ? html`<p>`: nothing}
    <div class="${classes}">

        <slot></slot>
    </div>
    ${p ? html`</p>` : nothing}
  

  `;
  }
}