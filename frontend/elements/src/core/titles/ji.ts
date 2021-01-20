import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';

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
        color: #5590fc;
        font-weight: 500;
        margin-right:36px;

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

  render() {

    const {size, color, bold, italic, weight} = this;

    const classes = classMap({ 
      [size]: true,
      [color]: true,
      bold: bold,
      italic: italic,
      [weight]:true,
    });
    return html`
    <div class="${classes}">
        <slot></slot>
    </div>
  

  `;
  }
}