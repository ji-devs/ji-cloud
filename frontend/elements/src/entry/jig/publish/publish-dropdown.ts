import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/entry/jig/publish/dropdown-list-object";
@customElement('publish-dropdown')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
        width: 419px;
        border-radius:16px;
        padding:32px;
        box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.25);
        background-color:#ffffff;
        position:relative;
        display:block;
     
        
    }
    h1{
        font-size: 24px;
        font-weight: 300;
        color:#fd6220;
        margin-bottom:16px;
    }
    .title{
        border-bottom:solid 1px #e2e5eb;
    }
 
    .closed {
        display:none;
    }
    img-ui{
        position:absolute;
        right:10px;
        top:10px;
    }
    .tooltip-wrapper.copied{
      display:block;
    }
    .tooltip-wrapper{
      display:none
    }
    
    `];
  }

  @property()
  title: string = "";

  
  @property({type:Boolean})
  closed: boolean = true;

  @property({type:Boolean})
  copied: boolean = false;

  render() {

    const {closed, copied} = this;

    const STR_DROPDOWNTITLE = "Select Share Option";

    return html`
     <main class="${closed ? 'closed' : ''}">
     <img-ui path="icn-x-close.svg"></img-ui>
      <div class="dropdown-wrapper">
        <div class="title">
            <h1>${STR_DROPDOWNTITLE}</h1>
        </div>
        <dropdown-list-object  class="list" mode="share"></dropdown-list-object>
          <dropdown-list-object class="list" mode="embed"></dropdown-list-object>
          <dropdown-list-object class="list" mode="link">
          </dropdown-list-object>
           <div class="${copied ? 'copied' : ''} tooltip-wrapper">
            <slot name="tooltip" ></slot>
        </div>
      </div>
    </main>
  `;
  }
}