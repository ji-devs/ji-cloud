import { MEDIA_UI } from '@utils/path';
import "@elements/core/lists/column-list";
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('footer-kidsafe')

export class _ extends LitElement {
  static get styles() {
    return [css`
  img-ui{
    width: 156px;
    height: 54px;
    object-fit: contain;
    margin-bottom:99px;
    display: block;
    margin-left:123px;

  }
  .wrapper{

display:flex;
  }
.stripe{
    width:1px;
    height: 20px;
    background-color: #ffffff; 
    opacity: 0.35;
    margin-left:16px;
    margin-right:16px;
    display: block;

}
column-list{
  list-style-type: none;
  margin-left:16px;
  display: block;
}
.line{
  display:flex;
  margin-top:25px;
}
    `]
  }



  render() {
     
    const STR_TERMS ="Terms & Conditions";
    const STR_PRIVACY = "Privacy Policy";

    return html`
     <div class="wrapper">
        <img-ui   path="logo-kid-safe.png"></img-ui>
        <div class="line">
        <column-list text_line="${STR_TERMS}" color="white" ></column-list>
        <div class="stripe"></div>
        <column-list text_line="${STR_PRIVACY}" color="white"></column-list>
        </div>
      </div>
  `;
  }
}