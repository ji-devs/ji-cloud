import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/entry/home/sections/footer-jigs";
import "@elements/entry/home/sections/products-section";
import "@elements/entry/home/sections/footer-help";
import "@elements/entry/home/sections/contactus-section.ts";
import "@elements/core/dividers/stripe-along";
import "@elements/entry/home/sections/whoweare-section.ts";
import "@elements/entry/home/sections/footer-productservices";
import "@elements/entry/home/TOSORT/footer-kidsafe"; 
@customElement('footer-section')
export class _ extends LitElement {

 static get styles() {
   return [css`
   .inside-wrapper{
      display:flex;
   }
  main{
   background-color: #192150;
   width: 1920px;
   height: 700px;
  }
   
  .title{
    display:flex;
    margin-bottom:-20px;

  }
  .footer-column{
   margin-left:90px;
    margin-top:20px;
 }
 .titleAbout{
   margin-top:50px;
   margin-left:600px;
 }
 .titleJi{
   margin-top:50px;
   margin-left:80px;
 }
 ::slotted([slot=button]){
   margin-top:80px;
 }
 .kidsafe{
  display:block;
  margin-top:-100px;
 }
 }
   `];
 }
 render() {
   const {} = this;
   const STR_TITLEJi="Jewish Interactive";
   const STR_TITLEAbout="About Us";
   return html`
   <main>
   <div class="title">  
      <title-section titlecolor="lightBlue" title="${STR_TITLEJi}" size="medium" class="titleJi"></title-section>
      <title-section titlecolor="lightBlue" title="${STR_TITLEAbout}" size="medium" class="titleAbout"></title-section>
    </div>
        <div class="inside-wrapper">
        <div class="footer-column">
         <footer-jigs></footer-jigs>
        </div>
        <div class="footer-column">
         <products-section></products-section>
        </div>
        <div class="footer-column">
         <footer-help></footer-help>
        </div>
        <div class="footer-column">
         <contactus-section></contactus-section>
        </div>
        <stripe-along class="footer-column"></stripe-along>
        <div class="footer-column">
         <whoweare-section></whoweare-section>
        </div>
        <div class="footer-column">
        <footer-productservices></footer-productservices>
        </div>
        <slot name="button"></slot>
        </div>
        <footer-kidsafe class="kidsafe" ></footer-kidsafe>    </main>
 `;
 }
}