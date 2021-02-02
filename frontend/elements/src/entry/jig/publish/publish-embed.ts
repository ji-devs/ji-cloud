import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('publish-embed')
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
        font-size: 16px;
        font-weight: 500;
        margin-bottom:16px;
    }
    .title{
      display:flex;
      justify-content:center;
    }
 
 
    .closed {
        display:none;
    }
    .close{
        position:absolute;
        right:10px;
        top:10px;
    }
    .embed{
      width: 371px;
      height: 164px;
      padding: 24px;
      border-radius: 8px;
      background-color: #f7f7f7;
    }
    .back{
      display:flex;
      position:absolute;
      top:10px;
      left:10px;
      align-items:center;
      cursor:pointer;
    }
    p{
      color: #5590fc;
      margin:0 0 5px;
    }
    .copy{
      display:flex;
      margin-top:12px;
      justify-content:center;
      position:relative;
      cursor:pointer;
    }
    ::slotted([slot="tooltip"]){
      right: 140px;
      position: absolute;
      top: -30px;
      
    }
    textarea, textarea:focus {
      border:none;
      outline:none;
      font-family: Poppins;
  font-size: 14px;
    }
    .tooltip-wrapper.copied{
      display:block;
    }
    .tooltip-wrapper{
      display:none
    }
    
    `];
  }

  @property({type:Boolean})
  closed: boolean = false;

  @property({type:Boolean})
  copied: boolean = false;

  render() {

    const {closed,copied} = this;
    const STR_BACK ="Back";
    const STR_EMBED = "Embed code:";
    const STR_COPY = "Copy Code";

    return html`
     <main class="${closed ? 'closed' : ''}">
     <img-ui class="close" path="icn-x-close.svg"></img-ui>
     <div class="back">
      <img-ui path="icn-arrow-small.svg"></img-ui>
      <p>${STR_BACK}</p>
     </div>
      <div class="dropdown-wrapper">
        <div class="title">
            <h1>${STR_EMBED}</h1>
        </div>
        <textarea class="embed"></textarea>
        <div class="copy">
          <img-ui path="icn-copy.svg"></img-ui>
          <p>${STR_COPY}</p>
          <div class="${copied ? 'copied' : ''} tooltip-wrapper">
            <slot name="tooltip" ></slot>
        </div>
        </div>
       
        
      </div>
    </main>
  `;
  }
}