import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('create-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
      display:flex;
      background-color:#fffde6; 
      width: 1920px;
      height: 600px;
  
  
     }

    .inside-wrapper{
       display:flex;
       margin-left:70px;
       margin-top: -90px;

    }
  
   
   img-ui{
    width: 323.7px;
    height: 354.9px;
    margin-top:-250px;
    margin-left:380px;
    width: 323.7px;
    height: 354.9px;
    position: absolute;

   }
   .gears{
  
    
  }
  ::slotted([slot=subtitle]){
     margin-top: 65px;
     margin-left:80px;
     display:block;
  
  }

  ::slotted([slot=title]){
    margin-left:10px;
    display:block;
 
 }

  ::slotted([slot=line]){
    display:flex;
    margin-top:16px;
    margin-left:80px;

    }

::slotted([slot=Start-creating]){
        margin-top:50px;
        display:block;
        margin-left:80px;

          
        }

.right_side{
    width: 853px;
    height: 600px;
    position: relative;
  }
.left_side{
    width: 1067px;
    height: 600px;

}


    `];
  }
  @property()
  STR_PATHGEARS:string = ""; 




  render() {

    const {} = this;
 
     return html`
    <main>
    <div class="left_side">
      <slot name="girl"></slot>
    </div> 

    <div class="right_side">
    <slot name="subtitle"></slot>
     <div class="inside-wrapper">
     <slot name="title"></slot>
    </div>

     <slot name="line"></slot>

    <slot name="Start-creating"></slot>
    <img-ui path="Illustration_Gears.jpg" class="gears"><img-ui>

    </div>


    </main>
  `;
  }
}