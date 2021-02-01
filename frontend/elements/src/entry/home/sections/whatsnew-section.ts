import "@elements/core/buttons/arrow-button";
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
     margin-top:247px;
      margin-left:10px;
 
   }

   .arrow{
       margin-top:247px;
        margin-right:45px;
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
 ::slotted([slot=contentpage]){
  display:block;
 margin-left:-110px;
 }

    `];
  }

   

  render() {

    const { } = this;

 

    return html`
    <main>
    <div class="header">
    <slot name="title"></slot>
    </div>


    <div class="wrapper">
      <arrow-button class="arrow"  color="blue" direction="left"></arrow-button>


    <slot name="contentpage"></slot>

     <arrow-button class="fliparrow"  color="blue" direction="right"></arrow-button>

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