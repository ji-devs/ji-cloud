import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('plain-black')
export class _ extends LitElement {
  static get styles() {
    return [css`
    p{
        color: #4a4a4a;
        margin-top:0;
       margin-bottom:0;
        

    }
    .bold {
      font-weight:500;
    }
    .numberhidden{
      display:none;
    }
    .number{
      display:block;
      margin-left:4px;
     
    }
    div{
      display:flex;
    }
    
   
    `];
  }

  @property()
  title:string = ""; 
  @property({type: Boolean})
  bold:boolean = false; 
  @property({type: Boolean})
  number:boolean = false; 
  @property({type: Number})
  amount:number = 16; 

  render() {

    const {title, bold, number, amount} = this;

    return html`
    <div>
    <p class="${bold ? 'bold' : ''}">${title}</p>
    <p class="${number ? 'number' : ''} bold numberhidden">${amount}</p>
    </div>
  `;
  }
}