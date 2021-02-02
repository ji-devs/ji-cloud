import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('aboutus-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;
        margin-left:566px;    
        margin-top:72px;    

    }

    
   main{
    background-color:#fd6b71    ; 
    width: 1920px;
    height: 783px;
     position: relative;

   }
   
  .title{
    display:block;
    margin-left:10px;
   }

 
  

   .content{
    display:block;
    margin-left:30px;

   }
   ::slotted([slot=cardparents]){
  margin-top:0px;
  margin-left:160px;
  z-index:1;

 

 }

   ::slotted([slot=cardteachers]){
   margin-top:100px;
   margin-left:0px;
   display:block;
   z-index:1;

 }

 
   .yellowimg{
    width: 276px;
    height: 246px;
    position: absolute;
    top:120px;
    left:30px;
    z-index:2;
   
   }
  
   .cards{
    display:flex;

   }

   .Shape_Teachers{
    width: 821px;
  height: 402px;
   position: absolute;
  z-index:0;
  left:60px;
  top:300px;


   }
   .Shape_Parents{
    width: 821.3px;
    height: 401.9px;
    position: absolute;
    z-index:0;
      right:80px;
    top:200px;

   }
    `];
  }



  render() {

 
   
    const STR_STARTTITLE = "What they say ";
    const STR_ENDTTITLE = "about us ";
  

    return html`
    <main>
    <div class="inside-wrapper">
    <title-section titlecolor="white"  title="${STR_STARTTITLE}" size="large"  class="title"></title-section>
    <title-section titlecolor="yellow" title="${STR_ENDTTITLE}" size="large" class="title"></title-section>
     </div>
    <img-ui class="yellowimg" path="yellow_square.png"></img-ui>
     <div class="cards">
     <slot name="cardteachers"></slot>
     <slot name="cardparents"></slot>
    
     <img-ui class="Shape_Teachers" path="Image_Shape_Teachers@2x.png"></img-ui>
     <img-ui class="Shape_Parents" path="Image_Shape_Parents@2x.png"></img-ui>

     </div>

  
        
    </main>
  `;
  }
}

