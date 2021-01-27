import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('dropdown-list-object')
export class _ extends LitElement {

  static get styles() {
    return [css`
    p{
    
        margin-left:12px;
        margin-top:0;
        margin-bottom:0;
  
    }
    .wrapper{
        display:flex;
        align-items:center;
        margin-top:16px;
        justify-content:space-between;
        padding-right:16px;
        position:relative;
    }
    .inside-wrapper{
        display:flex;
        align-items:center;
    }
    ::slotted([slot="tooltip"]){
        position:absolute;
        left: 110px;
    top: -30px;
    }
   
    `];
  }
@property()
label: string = "";
@property()
icon: string = "";

  

  render() {
    const {icon, label} = this;
   
   

    return html`
<div class="wrapper">
    <div class="inside-wrapper">
        <img-ui path="${icon}"></img-ui>
        <p>${label}</p>
    </div>
    <img-ui path="icn-record-activity-hover.svg"></img-ui>
    <slot name="tooltip"></slot>
</div>`
  }
}