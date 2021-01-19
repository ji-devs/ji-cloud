import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import {nothing} from "lit-html";
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

    .link {
      cursor: pointer;
    }
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

  @property({type: Boolean})
  p:boolean = false;

  @property({type: Boolean})
  link:boolean = false;

  render() {

    const {bold, italic, underlined, size, color, p, link} = this;

    const classNames = classMap({link});

    return html`
    ${p ? html`<p>`: nothing}
    <div class="${classNames}">
        <slot></slot>
    </div>
    ${p ? html`</p>` : nothing}
  

  `;
  }
}