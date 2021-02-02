import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";

@customElement('list-two-column')
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
    .column-wrapper{
      display:flex;
    }
  li{
      margin-bottom:12px;
  }
  p{
    color: #5590fc;
    font-weight:500;
    margin-top:0;
  }
    `];
  }

  @property()
  label?:string = ""; 

  render() {

    const {label} = this;

    return html`
    <main>
    ${label && label != "" ? html`<p>${label}</p>` : nothing}
    <ul>
    <div class="column-wrapper">
      <slot name="one"></slot>    
      <slot name="two"></slot>  
    </div>
    </ul>
    </main>
  `;
  }
}