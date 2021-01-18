import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('search-card-full')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
        width:354px;
        height:384px;
        border-radius:20px;
        display:none;
       
    }

    .front {
        display:flex;
        align-items:center;
        flex-direction:column;
        width:354px;
        height:384px;
    }

    .back{
        display:flex;
        background-color:#2565d5;
        width:354px;
        height:384px;
        padding:16px 24px;
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
    `];
  }

  @property({type:Boolean})
  front:boolean = false;



  render() {

    const {front} = this;

    return html`    
    <main class="${front ? 'front' : 'back'}">
    <slot name="image"></slot>
    <slot name="title"></slot>
    <div class="subtitle-wrapper">
    <slot name="subtitle"></slot>
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