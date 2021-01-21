import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('aboutus-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;
       margin-left:550px;    
    }

    
   main{
    background-color:#fd9087    ; 
    width: 1920px;
    height: 783px;
    display:block;
    position: relative;

   }
   
   ::slotted([slot=title]){
    display:block;
    margin-left:10px;
   }

   ::slotted([slot=imgteacher] ){
    display:block;
     margin-top:100px;
   }
 
   ::slotted([slot=imgparent]){
    display:block;
     margin-top:100px;
   }

   .content{
    display:block;
    margin-left:30px;

   }
 .right-side{
  margin-top:40px;
  margin-left:1011px;
  width: 822.3px;
  height: 403px;
  display:flex;

 }

 .left-side{
   margin-top:-280px;
   margin-left:100px;
  width: 822.3px;
  height: 403px;
  display:flex;

 }


   .rpoints , .lpoints{
     display:flex;
     margin-top:50px;

   }

   .arrow{
    margin-top:160px;
    margin-right:30px;

   }

   .fliparrow{
    transform: scaleX(-1);
    margin-top:160px;
    margin-left:30px;

   }
   .yellowimg{
    width: 275.5px;
    height: 245.4px;
    position: absolute;
    margin-top:0px;
    margin-left:50px;
   }
  
    `];
  }



  render() {

    const {} = this;

    const PATH_ARROW="icn-arrow.svg";


    return html`
    <main>
    <div class="inside-wrapper">
    <slot name="title"></slot>
    </div>
    <img-ui class="yellowimg" path="yellow_square.jpg"></img-ui>
     <div class="right-side">
     <img-ui class="arrow" path="${PATH_ARROW}"></img-ui>

       <slot name="imgparent"></slot>

       <div class="content">
          <slot name="title-sub-paragraph-right"></slot>
        <div class="rpoints">
           <slot name="rpoint"></slot>
        </div>

    </div>
    <img-ui class="fliparrow" path="${PATH_ARROW}"></img-ui>

</div>



 <div class="left-side">
  <img-ui class="arrow" path="${PATH_ARROW}"></img-ui>

 <slot name="imgteacher"></slot>

   <div class="content">
     <slot name="title-sub-paragraph-left"></slot>
   <div class="lpoints">
      <slot name="lpoint"></slot>
     </div>

 </div>
 <img-ui class="fliparrow" path="${PATH_ARROW}"></img-ui>

  </div>
        
    </main>
  `;
  }
}

// <div class="left-side">
//         <slot name="title-sub-paragraph"></slot>
      
//         <div class="4points">
//         <slot name="points"></slot>
//         </div>
        
//         </div>