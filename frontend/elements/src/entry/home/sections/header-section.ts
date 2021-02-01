 import "@elements/entry/home/TOSORT/column-list";
 import "@elements/core/buttons/rectangle";
 

import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('header-section')
export class _ extends LitElement {
  static get styles() {
    return [css`

   main{
    // width: 88px;
    height: 512px;
    display:flex;
   }

.logo{
  width: 115.4px;
  height: 40.1px;
  display:block;
  margin-left:25px;
  margin-top:29px;
  margin-right:37px;

}

::slotted([slot=menu-tab]){
  margin-left:8px;
 display:block;
}

::slotted([slot=icon]){
  margin-left:97px;
  margin-top:48px;

 display:block;
}

.button{
  display:block;
  margin-left:265px;
  margin-top:25px;
}
.userdetails{
  display:flex;

}
.username{
  display:block;
  margin-left:16px;
  margin-top:36px;
}
.imguser{
  display:block;
  margin-left:90px;
  margin-top:22px;
}
.arrow{
  display:block;
  margin-left:3px;
  margin-top:36px;
}
    `];
  }


 
 

  render() {

    const {} = this;
    const STR_DONATE="Donate"; 

    const STR_NAMEUSER="Shalom Corinne";
    const PATHIMGUSER="Image_User.png";
   
    return html`
    <main>
   <img-ui class="logo" path="Logo.png"></img-ui>
   <slot name="menu-tab"></slot>
   <button-rect class="button" size="small" color="green" bold="true">${STR_DONATE}</button-rect>
   <slot name="icon"></slot>

<div class="userdetails">
<img-ui class="imguser" path="${PATHIMGUSER}"></img-ui>
<column-list class="username" text_line="${STR_NAMEUSER}" ></column-list>
<img-ui class="arrow" path="Icn_Chevron_Hover.svg"></img-ui>
</div>

    </main>
  `;
  }
}