import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('pill-listitem')
export class _ extends LitElement {
  static get styles() {
    return [css`
div{
    width: 88px;
    padding: 3px 0;
    border-radius: 12px;
    border: solid 1px #5590fc;
    background-color: #c4dbff;
    display:flex;
    justify-content:center;
    align-items:center;
    font-size:14px;
}
    `];
  }



  @property()
  label:string = ""; 

  render() {

    const {label} = this;

    return html`
<div>${label}</div>

  `;
  }
}