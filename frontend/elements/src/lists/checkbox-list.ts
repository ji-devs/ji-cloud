import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('checkbox-list')
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
    `];
  }

  @property()
  label:string = ""; 
  @property()
  title:string = ""; 

  render() {

    const {label, title} = this;

    return html`
    <main>
    <p>${title}</p>
    <ul>
      <slot></slot>    
    </ul>
    </main>
  `;
  }
}