import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('menu-tab')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
        display:inline-block;
        margin-right: 8px;
        background-color:#ffffff;
        }
    :hover{
        background-color:#f3f8fe;
    }
    .tab-content-wrapper{
        padding: 0 24px 28px;
    }
    .tab-content{
        display:flex;
        align-content: center;
        padding-top:26px;
        
        
    }
    .menu-icon{
        margin-right:6px;
        
    }
    p{
        font-weight: 500; 
    }

    .selected .tab-content{
        color: #ed6065;
        border-top:solid 6px #fd7076;
      
    }
    `];
  }

  render() {
    return html`
    <main class="">
        <div class="tab-content-wrapper">
        <div class="tab-content">
        <img class="menu-icon" src="${MEDIA_UI}/icons/icn-menu-home.svg"/>
        <p><slot></slot></p>
        </div>
        </div>
        
    </main>
  `;
  }
}