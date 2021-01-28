 import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/buttons/circle";
import "@elements/core/buttons/arrow-button";
import "@elements/widgets/nav/steps-nav";
 
interface Item {
  imgPath: string;  
  title: string;
  subtitle: string;
  body: string;
}

  @customElement('moving-card')
export class _ extends LitElement {
  static get styles() {
    return [css`

   main{
    width: 822.3px;
    height: 403px;
    background-color:#fd9087;
   }
 
   

   .wrapper{
     display:flex;
   }

   button-circle{
     margin-left:-50px;
     display:block;
   }

   .footer{
    margin-left:80px;
    margin-top:80px;

    display:block;
    display:flex;

   }
    `];
  }

  @property()
  nItems: number = 0;

  @property()
  items: Array<Item>= new Array(5);

  @property()
  activeIndex: number=0;

  @property()
  activearrowright: boolean=true;

  @property()
  activearrowleft: boolean=true;



  render() {

    const {} = this;

const PATHTEACHER="Sara-Halberstadt.png";

    return html`
    <main>
<div class="wrapper"> 

<arrow-button direction="left"></arrow-button>
 <img-ui path="${PATHTEACHER}">  </img-ui>

 <arrow-button direction="right"></arrow-button>

 </div>


 <div class="footer">
     <button-circle color="white" size="small"></button-circle>
    <button-circle color="pink" size="small"></button-circle>
    <button-circle color="pink" size="small"></button-circle>
    <button-circle color="pink" size="small"></button-circle>
    <button-circle color="pink" size="small"></button-circle>
     </div>
    </main>
  `;
  }
}

function disablearrow(nItems:number,activeIndex: number){

  if(nItems-1>=activeIndex)
  {

  }

}