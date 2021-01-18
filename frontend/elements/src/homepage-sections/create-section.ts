import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('create-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;

    }
   main{
    display:flex;
    background-color:#fffde6; 
 width: 1920px;
  height: 600px;


   }
   
   img-ui{
    width: 323.7px;
    height: 354.9px;
    position: absolute;
    margin-top:215px;
    margin-right:40px;
   }
   .fliparrow{

   }

   .gears{
    width: 323.7px;
    height: 354.9px;
    
  }
  ::slotted([slot=subtitle]){
    // margin-top: 200px;
    // margin-bottom: 250px;    
  }
  ::slotted([slot=line]){
    display:flex;
    margin-top:16px;
      
    }

::slotted([slot=Start-creating]){
        margin-top:80px;
          
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
    <div class="left_side"><slot name="girl"></slot></div> 

    <div class="right_side">

    <slot name="subtitle"></slot>
     <div class="inside-wrapper">
     <slot name="title"></slot>
    </div>

     <slot name="line"></slot>

    <slot name="Start-creating"></slot>
    </div>


    // <img-ui path="Illustration_Gears.jpg" class="gears"><img-ui>

    </main>
  `;
  }
}