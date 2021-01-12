import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('menu-tab')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
        display:inline-block;
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

  @property()
  label: string = "";

  render() {

    const {label} = this;

    return html`
    <main class="">
       <slot name="one"></slot>
       <slot name="two"></slot>
    </main>
  `;
  }
}