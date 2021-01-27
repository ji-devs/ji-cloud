import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('publish-dropdown')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
        width: 419px;
        border-radius:16px;
        padding:32px 0 32px 32px;
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
    
    `];
  }

  @property()
  title: string = "";

  
  @property({type:Boolean})
  closed: boolean = false;
  

  render() {

    const {title, closed} = this;

    return html`
     <main class="${closed ? 'closed' : ''}">
     <img-ui path="icn-x-close.svg"></img-ui>
      <div class="dropdown-wrapper">
        <div class="title">
            <h1>${title}</h1>
        </div>
        <slot name="list"></slot>
       
        
      </div>
    </main>
  `;
  }
}