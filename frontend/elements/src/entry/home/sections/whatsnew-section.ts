import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('whatsnew-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
   

   main{
       background-color:#afcbf4;
       width: 1920px;
       height: 909px;
   }
   ::slotted([slot=title]){
  
    text-align: center;
    margin-top:72px;
    margin-right:749px;

   }

 
.fliparrow{
     transform: scaleX(-1);
     margin-top:247px;
   }

   .arrow{
       margin-top:247px;
   }

 
       .points{
        display:flex;
        margin-left:912px;
        margin-top:80px;
       }
       .wrapper{
           display:flex;
           align-items:center
           width:1920px;
           height: 540px;
       }
      .footer{
      width:1920px;
      height: 168px;
 
      }

 .header{
  width:1920px;
  height:201px;

 }





    `];
  }

  @property()
  PATH_ARROW:string = ""; 



  render() {

    const {} = this;

    const PATH_ARROW="icn-arrow.svg";


    return html`
    <main>
    <div class="header">
    <slot name="title"></slot>
    </div>

    <div class="wrapper">
     <img-ui class="arrow" path="${PATH_ARROW}"></img-ui>
<<<<<<< HEAD

=======
    
>>>>>>> naomi/master
    <slot name="contentpage"></slot>

    <img-ui class="fliparrow" path="${PATH_ARROW}"></img-ui>
    </div>

   <div class="footer">

   <div class="points">
   <slot name="point"></slot>
   </div>
   </div>
   
    </main>
  `;
  }
}