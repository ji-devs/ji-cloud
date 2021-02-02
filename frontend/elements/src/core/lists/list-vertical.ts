import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";

@customElement('list-vertical')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
      padding-left: 8px;
      padding-right: 80px;
    }
    ul{
      padding-left:0;
    }
  li{
      margin-bottom:12px;
  }
  p{
    color: #5590fc;
    font-weight:500;
    margin-top:0;
  }
  .error ul{
    border-right: solid 2px #e36486;
  }
  .error p{
    color:#e36486
  }
    `];
  }

  @property()
  label?:string = ""; 

  @property({type:Boolean})
  error:boolean = false; 

  render() {

    const {label,error} = this;

    return html`
    <main class="${error && "error"}">
    ${label && label != "" ? html`<p>${label}</p>` : nothing}
    <ul>
      <slot></slot>    
    </ul>
    </main>
  `;
  }
}