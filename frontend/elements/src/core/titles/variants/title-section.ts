import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('title-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
  h1{
    font-size: 64px;
    font-weight: 900;
  

  }
  .purple{
    color: #5662a3;
  }
  
  .lightBlue{
color:#6ca1fc;

  }

  .pink{
    
color:#fd7076;
  }

  .darkblue{
color:#5662a3;

  }

  .white{
    color:#ffffff;
  }
  
  


  #medium{
    font-size: 48px;
    font-weight: 900;
  }



 
    `];
  }


  @property()
  title:string = ""; 

  @property()
  titlecolor:string = "";

  @property()
  size:string = "";
  
 

  render() {
    const {title,titlecolor,size} = this;

    return html`
        <h1 class="${titlecolor}" id="${size}">${title}</h1> 
  `;
  }
}