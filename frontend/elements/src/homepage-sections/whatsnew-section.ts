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
    font-size: 64px;
    font-weight: 900;
    color:#5662a3;
    text-align: center;
    margin-top:72px;
    margin-right:749px;

   }

 
.fliparrow{
     transform: scaleX(-1);
    // margin-right:48px;
     margin-top:247px;
   }

   .arrow{
       margin-top:247px;
   }

   img-ui{
    // margin-top:80px;
    // margin-left:48px;
    // margin-right:48px;
    // border-bottom-right-radius: 25px;
    // border-top-right-radius: 25px;
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
        // margin-top:201px;
       }
 .footer{
  width:1920px;
  height: 168px;
 
 }

 .header{
  width:1920px;
  height:201px;

 }

 ::slotted([slot=contentpage]){
//  margin-right:122px;
 }



    `];
  }

  @property()
  PATH_ARROW:string = ""; 



  render() {

    const {} = this;

    const PATH_ARROW="icn-arrow.svg"


    return html`
    <main>
<div class="header">
    <slot name="title"></slot>
    </div>

    <div class="wrapper">
<img-ui class="arrow" path="${PATH_ARROW}"></img-ui>

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