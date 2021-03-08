import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing} from 'lit-html';
import { mediaUi } from '@utils/path';

export type Kind = "new" | "label" | "";

@customElement('card-front')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
        width:354px;
        height:384px;
        border-radius:20px;
        display:flex;
        align-items:center;
        flex-direction:column;
        position:relative;
       
    }
    ::slotted([slot="title"]){
        margin-top:16px;
        
       
    }
    ::slotted([slot="subtitle"]){
        height: 24px;
       
    }
    ::slotted([slot="age"]) {
        margin-left:8px;
        height:40px;
       
    }
    ::slotted([slot="language"]) {
        margin-right:8px;
        height:40px;
       
    }
    ::slotted([slot="dropdowns"]) {
        display:flex;
        align-items:center;
        width:100%;
        position:relative;
       
    }
    .subtitle-wrapper{
        display:flex;
    }
    .age-language{
        display:flex;
        align-items:center;
        margin-top:38px;
        justify-content:space-between;
        width:100%;
        
      
    }
    .dropdown-wrapper{
        display:flex;
        align-items:center;
        margin-top:26px;
        flex-direction:column;
        
    }
    .sub-wrapper{
        display:flex;
    }
    .sub-wrapper title-ji{
        margin-right: 4px;
    }
    .new{
        position:absolute;
        top: -46px;
        left: -10px;
    }
    .label{
        position:absolute;
        width: 32px;
        height: 64px;
        box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
        display:flex;
        flex-direction:column;
        align-items:center;
        justify-conent:center;
        text-align:center;
        top:-5px;
        right:35px;
    }
    .label h2{
        font-size: 20px;
        font-weight: bold;
        color:#606060;
        padding:0;
        margin:5px 0 0 0;
    }
    .label p{
        margin:0;
        font-size: 12px;
        font-weight: bold;
        color:#606060;
    }
    `];
  }

@property()
type:Kind = "";


@property()
bannernumber:number = 16;

  render() {
      const {type,bannernumber} = this;
    const STR_PLAYED = "Played";
    const STR_LIKED ="Liked";
    const STR_JIGS = "JIGs";

    const icon = type === "new" ? html`<img-ui path="New@2x.png" class="new"></img-ui>`
    : type === "label" ? html`<div class="label" style="background-image:url('${mediaUi("JIGs_units.svg")}')">
    <h2>${bannernumber}</p>
    <p>${STR_JIGS}</p>
    </div>`
    : nothing;
 

    return html`    
    <main>
    <div>${icon}</div>
    <slot name="image"></slot>
    <slot name="title"></slot>
    <div class="subtitle-wrapper">
        <div class="sub-wrapper">
            <title-ji color="black">${STR_PLAYED}</title-ji>
            <slot name="played"></slot>
        </div>
    <slot name="subtitle"></slot>
    <div class="sub-wrapper">
            <title-ji color="black">${STR_LIKED}</title-ji>
            <slot name="liked"></slot>
        </div>
    </div>
    <div class="dropdown-wrapper">
    <slot name="dropdowns"></slot>
    </div>
    
    <div class="age-language">

        <slot name="age"></slot>
        <slot name="language"></slot>
    </div>
    <slot name="banner"></slot>
    
    </main>

  `;
  }
}