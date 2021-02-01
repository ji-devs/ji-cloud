  import { LitElement, html, css, customElement, property } from 'lit-element';
 import "@elements/core/titles/variants/title-section";
 import "@elements/core/buttons/rectangle" ;
 import "@elements/entry/home/TOSORT/column-list";

  @customElement('logout-section')
 export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;
    }
   main{
    width: 576px;
    height: 352px;
    position:relative;

  }
    
  
  .title{
 left:40px;
    
    position:relative;
    top:86px;
  }
  
  .line{
    margin-left:40px;
    margin-top:7px;
    display:block;
  }
  .button{
    right:40px;
    bottom:40px;
     position:absolute;

  }

  .textbutton{
    left:40px;
    bottom:50px;
     position:absolute;

  }

   img-ui{
    width: 193px;
    height: 118px;
    position:absolute;
    right:104px;

   }

  .lines{
    margin-top:100px;

  }
 

    `];
  }
  render() {
    const {} = this;


    const STR_TITLE="Logout";
     const STR_IFYOU="If you are using a public computer, remember to";
     const STR_LOG=" log out when you’re done.";
     const STR_DOYOU="Do you want to logout? ";
    const STR_LOGOUT="Logout";
    const STR_Cancel="Cancel";

    return html`
    <main>
    <img-ui path="yellow_Illustration.png"></img-ui>
    <title-section titlecolor="darkblue" title="${STR_TITLE}" size="small" class="title"></title-section>

         <div class="lines">

        <column-list class="line" text_line="${STR_IFYOU}" size="medium"></column-list>
     <column-list class="line" text_line="${STR_LOG}" size="medium"></column-list>
     <column-list class="line" text_line="${STR_DOYOU}" size="medium"></column-list>

        </div>
         <div class="inside-wrapper">
         <button-rect size="large" color="blue" class="button">${STR_LOGOUT}</button-rect>
         <button-text  size="large"  color="blue" class="textbutton" >${STR_Cancel}</button-text>
     

         </div>

    </main>
  `;
  }
 }