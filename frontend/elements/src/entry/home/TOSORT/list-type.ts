import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('list-type')
export class _ extends LitElement {
  static get styles() {
    return [css`
  
   main{
     
   }
   
   ul{
    margin:0;
    padding:0;
color:orange;
   }

//    li{
//     font-size: 20px;
//     font-weight: normal;
//     margin-top:10px;
//    }
 

  
  

    `];
  }


  @property()
  line1:string = ""; 
//   @property()
//   line2:string = ""; 
//   @property()
//   line3:string = ""; 
//   @property()
//   line4:string = ""; 


 

  render() {

 const line1="eeeeeeeeeeeeee";
const line2="eeeeeeeeeeeeee"
const line3="eeeeeeeeeeeeee"
const line4="eeeeeeeeeeeeee"

    return html`
    <main>
   <ul>
   <slot name="list"></slot>
// <li>${line1}</li>
// <li>${line2}</li>
// <li>${line3}</li>
// <li>${line4}</li>


    </ul>
 
     </main>
  `;
  }
}