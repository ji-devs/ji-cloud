import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/buttons/text";


@customElement('jig-panel')
export class _ extends LitElement {
  static get styles() {
    return [css`
    
    
   main{
    width: 416px;
    height: 1402px;
    position:relative;

   }
  
.jig-head{ 
   display:block;
   margin-left:170px;
   margin-top:30px;
   z-index:1;

   }

  .jig-neck{
    display:block;
    margin-left:225px;
      margin-bottom:-20px;
      margin-top:-30px;
      z-index: -1;

  }
.rightHand{
  position:absolute;
   left:311px;
   top:250px;
}

.leftHand{
  position:absolute;
   left:211px;
   top:250px;
}
.tail{
  display:block;
  margin-left:155px;
  margin-top:-15px;
}

.logo{
  position:absolute;
  left:25px;
   top:30px;
}

.seemyji{
  display:block;
  margin-left:290px;
   margin-top:30px;
}


input[type=text]{
  width: 376px;
height: 39px;
border: solid 1px #89b3ff;
border-radius: 8px;
display:block;
  margin-left:10px;
   margin-top:50px;
 font-size: 16px;
  font-weight: normal;
  color:#a9b1b5;
}

    `];
  }


 

  render() {

    const {  } = this;

     const STR_SEE="See my JIGs"
     const STR_PLACEHOLDER="My JIG’s name"


    return html`
    <main>
    <img-ui class="logo" path="Logo.png"></img-ui>
    <button-text class="seemyji" color="blue" size="large">${STR_SEE}</button-text>
    <input type="text" placeholder="${STR_PLACEHOLDER}"  style=background-image:url("Icn_Edit.svg")  >
    <img-ui class="jig-head" path="Asset 1@2x.png"></img-ui>
    <img-ui  class="jig-neck" path="Path 148075.svg"></img-ui>
    <img-ui class="rightHand" path="Group 14809.svg"></img-ui>
     <slot name="leftHand"></slot>
    <slot name="jig-squad"></slot>
    <img-ui class="tail" path="Group 10810.svg"></img-ui>
    </main>
  `;
  }
}