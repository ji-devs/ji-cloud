import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';


@customElement('dropdown-tree-child')
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
    ul.closed {
      display: none;
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
      <img-ui class="px-1" path="icon-chevron-categories-24-px.svg" alt=""></img-ui>
      <div class="inside">
    </div>
    </div>
    <div data-id="label">${label}</div>
    <ul class="${open ? 'open' : 'closed'}">
            <slot></slot>
        </ul>
  </li>

  `;
  }
}