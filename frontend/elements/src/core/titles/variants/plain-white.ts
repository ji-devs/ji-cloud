import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('plain-white')
export class _ extends LitElement {
  static get styles() {
    return [css`
    p{
        color: #ffffff;
        margin-top:0;
        margin-bottom:0;
        word-wrap:normal;
        

    }
    
    .bold {
      font-weight:600;
    }
    .small{
      font-size:14px;
    }
    
   
    `];
  }

  @property()
  title:string = ""; 
  @property({type: Boolean})
  bold:boolean = false; 
  @property({type: Boolean})
  small:boolean = false; 

  render() {

    const {title, bold,small} = this;

    return html`
    <p class="${bold ? 'bold' : ''} ${small ? 'small' : ''}">${title}</p>
  `;
  }
}