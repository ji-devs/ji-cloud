import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
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
  @property()
  title:string = ""; 

  render() {

    const {title} = this;

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