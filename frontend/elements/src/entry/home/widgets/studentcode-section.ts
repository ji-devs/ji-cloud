  import { LitElement, html, css, customElement, property } from 'lit-element';
  import "@elements/core/titles/variants/title-section";
  import "@elements/core/dividers/square-divider";

import { nothing } from 'lit-html';
 
 export type Image =  "hidden"| "play"| "askforhelp" |"tryagain" ;


  @customElement('studentcode-section')
 export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;
       margin-left:170px;
       margin-top:130px;
    }
   main{
    width: 760px;
    height: 462px;
    position:relative;
  }
    
  
  .title{
    position:relative;
    text-align: center;
    top:86px;


  }
  
  .img{
    bottom:-5px;
    left:0px;
     position:absolute;
  }
 
 
  .square{
     margin-left:32px;
     display:block;
  }

    `];
  }
  @property()
  kindimage:Image = "hidden"; 
  
 
 
  render() {
    const {kindimage } = this;
    const STR_TITLE="Enter your student code";

    const path = kindimage === "play" ? "Image_JIG_StudentCode%402x.png"
        : kindimage === "askforhelp" ? "Image_Jig_Studentcode_error2@2x.png"
        : kindimage ==="tryagain" ? "Image_Jig_Studentcode_error1@2x.png";
 
         const  imgVisible = kindimage === "hidden"? true:false;
        
     return html`
    <main>
         <title-section titlecolor="darkblue" title="${STR_TITLE}" size="small" class="title"></title-section>

          <div class="inside-wrapper"> 
           <square-divider colorborder="small" size="blue" class="square"></square-divider>
          <square-divider colorborder="small" size="blue" class="square"></square-divider>
          <square-divider colorborder="small" size="blue" class="square"></square-divider>
          <square-divider colorborder="small" size="blue" class="square"></square-divider>
          </div>
         <img-ui class="img" path="${path}" hidden="${imgVisible}"> </img-ui>
       

    </main>
  `;
  }
 }

   
