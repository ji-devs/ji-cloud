import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from "lit-html";

export type Mode = "share" | "embed" | "link";
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
mode: Mode = "share"

  render() {
    const {mode} = this;
    const STR_SHARE ="Share with Students";
    const STR_URL ="Copy URL";
    const STR_EMBED ="Embed this JIG";
    const iconPath = mode === "share" ? "icn-student.svg"
      : mode === "embed" ? "Icn_Embed.svg"
      : mode === "link" ? "icn-url.svg"
      : nothing;
      
     const label = mode === "share" ? STR_SHARE
      : mode === "embed" ? STR_EMBED
      : mode === "link" ? STR_URL
      : nothing;
   

    return html`
    <div class="wrapper">
        <div class="inside-wrapper">
            <img-ui path="${iconPath}"></img-ui>
            <p>${label}</p>
        </div>
        <img-ui path="icn-record-activity-hover.svg"></img-ui>
        <slot name="tooltip"></slot>
    </div>`
  }
}