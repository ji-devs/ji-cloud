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
 
     }

    
     .img{
          width: 940px;
          height: 540px;
          display:block;
          border-radius: 25px 0px 0px 25px;
          // background-image: url("Sara-Halberstadt.jpg");
          background-color:#fffde6;


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

     .lines{
        // width: 604px;
        // height: 119px;
        // margin-right:56px;
        // margin-top:-144px;
        // display:block;
    }

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
    // <slot name="img"></slot>
     <div class="img"></div>

    

     <div class="content">
     <slot name="subtitle"></slot>
     <div class="lines">
     <slot name=line></slot>
     </div>
     <slot name="button"></slot>
     </div>

    </div>
        
  `;
  }
}
    //  <img-ui path="${pathimg}"></img-ui>
