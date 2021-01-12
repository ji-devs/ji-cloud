import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';


@customElement('tree-inside-child')
export class _ extends LitElement {

  static get styles() {
    return [css`
    li{
        position:relative;
        display:flex;
        content-items:center;

    }
    .open img-ui{
        transform: rotate(90deg);
    }
   
    .inside {
      position:absolute;
      border:solid 1px #c4dbff;
      border-right:none; 
      border-top:none;
      width:26px;
      height:24px;
      left:-28px;
      top: -10px


    }
    .icon-wrapper{
      position:relative;
      
    }
    .titleoptions{
      margin-top:8px;
      position: relative;
      top: 6px;
      
    }
    `];
  }

@property()
label: string = "";

@property({type: Boolean})
open: boolean = false;

  render() {

    const {label, open} = this;

    return html`
  
  <li class="titleoptions open">
    
    
    <div class="icon-wrapper">
      <div class="inside">
    </div>
    </div>
    <div data-id="label">${label}</div>
    <ul>
            <slot></slot>
        </ul>
  </li>

  `;
  }
}