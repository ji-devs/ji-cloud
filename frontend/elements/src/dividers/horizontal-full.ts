import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';

@customElement('horizontal-full')
export class _ extends LitElement {

  static get styles() {
    return [css`
    div{
        height:100%;
        width:2px;
        background-color:#707070;
        margin-left:14px;
        margin-right:14px;
    }
    `];
  }




  render() {

    const {} = this;

    return html`
    <div>

    </div>
  `;
  }
}