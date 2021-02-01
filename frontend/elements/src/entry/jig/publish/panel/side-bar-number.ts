 import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/lists/column-list";


@customElement('side-bar-number')
export class _ extends LitElement {
  static get styles() {
    return [css`
    
    
   main{
     width: 120px;
    height: 168px;
      
   }
   img-ui{
     display:block;
     margin-top:15px;
     margin-left:10px;
   }
   
 .title{
   margin-top:24px;
   display:block;
 }
 column-list{
  display:block;
   margin-left:10px;
 }

 ::slotted([slot=subtitle]){
  margin-left:10px;
  display:block;
}

    `];
  }


  @property()
  path:string = ""; 
 
  @property()
  title:string = ""; 



 

  render() {

    const {path,title} = this;
     


    return html`
    <main>

    <column-list text_line="${title}" size="medium" bold=true class="title"></column-list>
 

    <slot name="subtitle"> </slot>

    <img-ui path="${path}"></img-ui>
    
    </main>
  `;
  }
}