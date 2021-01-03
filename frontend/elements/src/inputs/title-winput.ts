import { LitElement, html, css, customElement, property } from 'lit-element';


@customElement('title-winput')
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

  @property()
  title: string = "";

  @property()
  path: string = "";

  render() {

    const {title} = this;
    return html`
    <div>
        <p>${title}</p>
        <slot name="input"></slot>
    </div>
  

  `;
  }
}