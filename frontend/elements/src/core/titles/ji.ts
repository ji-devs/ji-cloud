import { LitElement, html, css, customElement, property } from 'lit-element';

export type Color = "red" | "blue" | "white" | "green";
export type Size = "small" | "medium" | "large";

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
    `];
  }

  @property({type: Boolean})
  bold:boolean = false;

  @property({type: Boolean})
  italic:boolean = false;

  @property({type: Boolean})
  underlined:boolean = false;

  @property({type: Boolean})
  size:Size = "medium";

  @property({type: Boolean})
  color:Color = "red";

  render() {

    const {title} = this;
    return html`
    <div>
        <slot></slot>
    </div>
  

  `;
  }
}