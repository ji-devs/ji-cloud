 import "@elements/core/lists/column-list";
 import "@elements/core/buttons/rectangle";
 import "@elements/core/cards/icon";
 import "@elements/core/menu/menu-tab";



import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('header-section')
export class _ extends LitElement {
  static get styles() {
    return [css`

   main{
     height: 512px;
    display:flex;
   }

.logo{
   margin-left:25px;
  margin-top:29px;
  margin-right:37px;

}

.menu-tab{
  margin-left:8px;
 }

.icon{
  margin-left:97px;
  margin-top:48px;
 }

.button{
  margin-left:265px;
  margin-top:25px;
}
.userdetails{
  display:flex;

}
.username{
   margin-left:16px;
  margin-top:36px;
}
.imguser{
   margin-left:90px;
  margin-top:22px;
}
.arrow{
   margin-left:8px;
  margin-top:36px;
}
    `];
  }

  @property()
  UserName: string = " ";

 
 

  render() {
   const {UserName}=this;
     const STR_DONATE="Donate"; 

    const PATHIMGUSER="Image_User.png";
    const STR_STUDENTCODE="Student Code";
    const STR_HOME="Home";
    const STR_CONTENT="Content";
    const STR_CREATE="Create";
    const STR_COMMUNITY="Community";
    const STR_CLASSROOM="Classroom";
    const STR_ABOUTJI="About JI";
    const PATHHOME="icn-menu-home.svg";
    const PATHCONTENT="Icn_Menu_Content.svg";
    const PATHCREATE="Icn_Menu_Create.svg";
    const PATHCOMMUNITY="Icn_Menu_Community.svg";
    const PATHCLASSROOM="Icn_Menu_Classroom.svg";
    const PATHABOUTJI="Icn_Menu_About.svg";
   
   
    return html`
    <main>
   <img-ui class="logo" path="Logo.png"></img-ui>


   <div class="menu-tab">
   <menu-tab  uiIconPath ="${PATHHOME}">${STR_HOME}</menu-tab>
   </div>
  <div class="menu-tab">
  <menu-tab  uiIconPath ="${PATHCONTENT}">${STR_CONTENT}</menu-tab>

   </div>
  <div class="menu-tab">
  <menu-tab  uiIconPath ="${PATHCREATE}">${STR_CREATE}</menu-tab>

   </div>
  <div class="menu-tab">
  <menu-tab  uiIconPath ="${PATHCOMMUNITY}">${STR_COMMUNITY}</menu-tab>

   </div>
  <div class="menu-tab">
  <menu-tab  uiIconPath ="${PATHCLASSROOM}">${STR_CLASSROOM}</menu-tab>

   </div>
  <div class="menu-tab">
  <menu-tab  uiIconPath ="${PATHABOUTJI}">${STR_ABOUTJI}</menu-tab>

   </div>
  
  

   <button-rect class="button" size="small" color="green" bold="true">${STR_DONATE}</button-rect>
   <div class="icon">

 <card-icon label="${STR_STUDENTCODE}"  icon="group" />
</div>
 
<div class="userdetails">
<img-ui class="imguser" path="${PATHIMGUSER}"></img-ui>
<column-list class="username" text_line="${UserName}" ></column-list>
<img-ui class="arrow" path="icn_chevron_user.svg"></img-ui>
</div>

    </main>
  `;
  }
}