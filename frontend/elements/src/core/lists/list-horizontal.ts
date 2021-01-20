import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('list-horizontal')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
      padding-left: 8px;
      padding-right: 80px;
    }
    ul{
      padding-left:0;
      display:flex;
      margin-bottom:12px;
    }
  ::slotted(*){
     margin-right:18px;
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

  render() {

    const {label} = this;

    return html`
    <main>
    <p>${label}</p>
    <ul>
      <slot></slot>    
    </ul>
    </main>
  `;
  }
}