import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import {nothing} from "lit-html";
export type Color = "red" | "blue" | "white" | "green" | "lightgreen" | "lightblue" | "black";
export type Size = "small" | "medium" | "large" | "x-large";
export type Weight = "thin" | "bolder" | "x-bold";
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

    .link {
      cursor: pointer;
    }

    .blue {
      color: #5590fc;
    }
    .thin{
      font-weight:300
    }
    .black{
      color: #4a4a4a
    }
    .lightgreen{
      color: #9cddb2
    }
    .x-large{
      font-size:64px;
    }
    .large{
      font-size:32px;
    }
    .x-bold{
      font-weight:900
    }
    .lightblue{
      color:#e7f0fe;
    }
    .medium{
      font-size:16px;
    }
    .bold{font-weight:600}
    `];
  }

  @property({type: Boolean})
  bold:boolean = false;

  @property({type: Boolean})
  italic:boolean = false;

  @property({type: Boolean})
  underlined:boolean = false;

  @property()
  size:Size = "medium";

  @property()
  color:Color = "red";

  @property()
  weight:Weight = "x-bold";

  @property({type: Boolean})
  p:boolean = false;

  @property({type: Boolean})
  link:boolean = false;

  render() {

    const {size, color, bold, italic, weight, p, link} = this;

    const classes = classMap({ 
      [size]: true,
      [color]: true,
      bold: bold,
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