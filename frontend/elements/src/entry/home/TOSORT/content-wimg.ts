import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('content-wimg')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
        display:flex;
        width: 1676px;
        height: 540px;
        background-color:#ffffff ;
        margin-left:80px;
        border-radius: 25px;
        box-shadow: 0 3px 20px 0 rgba(0, 0, 0, 0.16);
        overflow:hidden;
 
     }

    
     .img{
          width: 940px;
          height: 540px;
          display:block;
          
          


       }
    
       .content{
        width: 610px;
        height: 440px;
        display:block;
        margin-left:70px;
        margin-top:25px;


    }
    
    ::slotted([slot=subtitle]){
      margin-top:50px;
       margin-left:10px;
       display:block;

     }

<<<<<<< HEAD:frontend/elements/src/entry/home/TOSORT/content-wimg.ts
     .lines{
        // width: 604px;
        // height: 119px;
        // margin-right:56px;
        // margin-top:-144px;
        // display:block;
    }
=======

    ::slotted([slot=button]){
     margin-top:200px;
     display:block;
  }

  ::slotted([slot=line]){
    margin-top:8px;
    display:block;
 }
>>>>>>> naomi/master:frontend/elements/src/entry/home/TOSORT/contet-wimg.ts

    ::slotted([slot=button]){
     margin-top:200px;
     display:block;
  }

  ::slotted([slot=line]){
    margin-top:8px;
    display:block;
 }


  
    `]
  }

  @property()
  pathimg:string = ""; 
 
 
 

  render() {

    const {pathimg} = this;

    return html`
    <div class="inside-wrapper">
    
<<<<<<< HEAD:frontend/elements/src/entry/home/TOSORT/content-wimg.ts
     <div class="img"></div>
=======
    <img-ui path="image_News.png"></img-ui>
>>>>>>> naomi/master:frontend/elements/src/entry/home/TOSORT/contet-wimg.ts

    

     <div class="content">
     <slot name="subtitle"></slot>
     <div class="lines">
     <slot name=line></slot>
     </div>
     <slot name="button"></slot>
     </div>
<<<<<<< HEAD:frontend/elements/src/entry/home/TOSORT/content-wimg.ts
     //  <img-ui path="${pathimg}"></img-ui>
=======
    
>>>>>>> naomi/master:frontend/elements/src/entry/home/TOSORT/contet-wimg.ts

    </div>
        
  `;
  }
}
