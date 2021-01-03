import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';


@customElement('tree-dropdown')
export class _ extends LitElement {

  static get styles() {
    return [css`
    .main-wrapper{
        border-color:#e6f0ff;
        border-style:solid;
        border-width: 2px 2px 2px 8px;
        width:848px;
        min-height:48px;
        border-radius:12px;
        
        
    }
    .inside-wrapper{
        display:flex;
        align-items:center;
        
    }
    .text-wrapper{
        display:flex; 
        height:48px;
        align-items:center;
    }
    ::slotted([slot=children]) {
        padding:0 14px 4px 14px;
        margin-top: 8px;
        margin-left:16px;
    }
    img {
        margin: 0 8px;
    }
    .open img{
        transform: rotate(90deg);
    }
    `];
  }

@property()
label: string = "";

@property()
path: string = "";

@property()
open: boolean = false;

  render() {

    const {label, path, open} = this;

    return html`
  
    <div class="main-wrapper ${open}">
        <div class="inside-wrapper">
            <div class="text-wrapper flex py-3">
                <img-ui class="" path="${path}" alt=""></img-ui>
                <p>${label}</p>
            </div>

        </div>
        <ul>
            <slot name="children"></slot>
        </ul>
    </div>

  `;
  }
}