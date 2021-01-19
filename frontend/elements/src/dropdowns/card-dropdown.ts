import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('card-dropdown')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
      display:flex;
      overflow:hidden;
      scroll:auto;
      justify-content:space-between;
     
      width:100%;
      border-bottom: solid 1px #3c7df0;
      
       
    }
    .collapsed{
        height:44px;
        overflow:hidden;
    }
    .expanded{
        height:100%;
    }
    .collapsed ::slotted([slot=content]){
        display:none;
    }
    .expanded img-ui{
      transform: rotate(180deg)
    }
    .content-inside{
      display:flex;
      flex-wrap: wrap;
    }
    .title-wrapper{
      display:flex;
      flex-wrap: wrap;
      margin-bottom:10px;
      padding-top:10px;
      
    }
    ::slotted([slot=title]){
      display:block;
      margin-bottom:10px;
    }
    img-ui{
      margin-top:10px;
      cursor:pointer;
    }
    
   
    `];
  }

  @property({type:Boolean})
  collapsed: boolean = true;

  

  @property()
  label: string = "";

  render() {

    const {collapsed,label} = this;

    return html`
     <main class="${collapsed ? 'collapsed' : 'expanded'}">
        <div class="title-wrapper">
      
          <slot name="title">${label}</slot>
        </div>
        <div class="content-inside">
          <slot name="content"></slot>
        </div>
       
        
        <img-ui path="Icn_Chevron_White.svg"></img-ui>
    </main>
  `;
  }
}