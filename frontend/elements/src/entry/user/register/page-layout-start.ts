import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import {Strength as PasswordStrength} from "@elements/entry/user/register/widgets/password-strength";

@customElement('page-layout-start')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
        display:flex;
    
    }
    .inside-wrapper{
        width:296px;
    }
    .side-image{
        width: 480px;
        min-width:300;
        min-height: 100vh;
        background-color: #def4ff;
        background-image: url('https://i.ibb.co/g9N7MLy/shapes-1.png');
        background-repeat: no-repeat;
        background-attachment: inherit;
        background-position: center;
    }
    .content-wrapper{
        padding:80px;
        position:relative;
    }
    h1{
        font-size: 32px;
        font-weight: 900;
        color:#5662a3
    }
    ::slotted([slot=google]){
        margin-bottom:20px;
    }
    ::slotted([slot=input]){
        margin-top:20px;
    }
    ::slotted([slot=passwordreminder]){
        text-align: end;
    }
    ::slotted([slot=submit]){
        margin-top:40px;
        margin-bottom: 24px;
    }
   
  
    .spacer{
        height:20px;
    }
    .text-hidden {
        display:none;
    }
    .password-wrapper {
        position: relative;
    }
    .password-wrapper div {
        position: absolute;
        top: 33%;
        right: -76px;
    }
    ::slotted([slot=contact]){
        position:absolute;
        bottom:20px;
        white-space:nowrap;
    }
    .account-wrapper{
        display:flex;
        align-items:center;
    }
    ::slotted([slot=noaccount]:last-child){
        margin-left:4px;
    }
    ::slotted([slot=sub]){
        white-space: nowrap;
    }
   
    `];
  }

  @property()
  title:string = ""; 

  @property()
  passwordStrength: PasswordStrength = "none";

  render() {

    const {title, passwordStrength} = this;


    return html`
    <div class="wrapper">
    <div class="side-image"></div>
    <div class="content-wrapper">
    <h1>${title}</h1>
    <slot name="sub"></slot>
    <div class="inside-wrapper">
        <slot name="google"></slot>
        <slot name="divider"></slot>
        <slot name="username"></slot>
        <div class="spacer"></div>
        <password-strength strength ="${passwordStrength}"></password-strength>
        <div class="password-wrapper">
        
        <slot name="password"></slot>
        <div>
        ${strengthText(passwordStrength)}
        </div>

        </div>
        <p>&nbsp;</p>

        <slot name="submit"></slot>
       
      </div>
      <div class="account-wrapper">
      <slot name="noaccount"></slot>
      </div>
      <slot name="contact"></slot>
  </div>
  
</div>
  `;
  }
}

function strengthText(mode: PasswordStrength) {
    const strengthlabel = mode === "weak" ? "Weak"
      : mode === "average" ? "Average"
      : mode === "strong" ? "Strong"
      : "";
    
      return html`<p>${strengthlabel}</p>`
    
  }