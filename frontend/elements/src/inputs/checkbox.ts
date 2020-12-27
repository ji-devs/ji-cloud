import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('input-checkbox')
export class _ extends LitElement {
  static get styles() {
    return [css`
    label{
        display: flex;
        align-items: baseline;
    }
    input{
        margin-left: 2px;
        margin-right: 1px;
        display:inline-block;
    }
    span{
        margin-left:12px;
    }
    li{
      list-style-type: none;
    }
    `];
  }



  @property()
  label:string = ""; 

  render() {

    const {label} = this;

    return html`
    <li>
      <label class="">
          <input type="checkbox"/>
              <span class="">
              ${label}
              </span>
      </label>
    </li>
  `;
  }
}