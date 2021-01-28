 import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('jiggling-body')
export class _ extends LitElement {
  static get styles() {
    return [css`
    
    
   main{
    width: 218px;
    height: 123px;
    background-color:#d8e7f9;
    border-radius:16px;
    border: solid 2px #d8e7f9;

   }
   img-ui{
     display:block;
     margin-top:42px;
     margin-left:89px;
   }
   
 
  

    `];
  }


  @property()
  path:string = ""; 


 

  render() {

    const {path} = this;

     


    return html`
    <main>
    <img-ui path="${path}"></img-ui>
    
    </main>
  `;
  }
}