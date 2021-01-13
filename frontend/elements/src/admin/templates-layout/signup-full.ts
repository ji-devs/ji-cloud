import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('signup-full')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
        display:flex;
    
    }
    .inside-wrapper{
        width:624px;
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
    }
    h1{
        font-size: 32px;
        font-weight: 900;
        color:#5662a3
    }
    ::slotted([slot=topleft]), ::slotted([slot=topright]),::slotted([slot=bottomleft]), ::slotted([slot=bottomright]){
       max-width:296px;
       min-width:296px;
       margin-bottom:40px;
    }
    ::slotted([slot=topleft]),::slotted([slot=bottomleft]){
        margin-right:32px;
    }
    ::slotted([slot=input]){
        margin-top:20px;
    }
    ::slotted([slot=passwordreminder]){
        text-align: end;
    }
    ::slotted([slot=submit]){
        margin-top:40px;
    }
    ::slotted([slot=noaccount]:last-child){
        margin-left:4px;
    }
    .noaccount{
        display:flex;
        align-items:center;
        margin-top:16px;
    }
    .two-row{
        display:flex;
        
    }
  
  
    .spacer{
        height:20px;
    }
    .hidden {
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
   
   
    `];
  }

  @property()
  title:string = ""; 

  @property()
  subtitle:string = ""; 

  @property()
  hidden:boolean = true; 

  render() {

    const {title, hidden, subtitle} = this;

    return html`
 <div class="wrapper">
  <div class="side-image">
  </div>
  <div class="content-wrapper">
    <h1>${title}</h1>
    <slot name="subtitle"></slot>
    <div class="inside-wrapper">
       <div class="two-row">
            <slot name="topleft"></slot>
            <slot name="topright"></slot>
        </div>
        <div class="two-row">
            <slot name="bottomleft"></slot>
            <slot name="bottomright"></slot>
        </div>
        <slot name="username"></slot>
        <div class="spacer"></div>
        <slot name="passwordstrength"></slot>
        <div class="password-wrapper">
        
        <slot name="checkbox">
            
        </slot>
        
        <slot name="passwordreminder"></slot>
        
        <slot name="submit"></slot>
       
      </div>
      <div class="noaccount">
        <slot name="noaccount"></slot>
      </div>
  </div>
</div>
  `;
  }
}