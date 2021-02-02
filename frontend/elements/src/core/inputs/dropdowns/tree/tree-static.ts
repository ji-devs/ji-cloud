import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
import { noChange } from "lit-html";

@customElement("dropdown-tree-static")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
 
        .inside-wrapper{
            display:flex;
            align-items:center;
            padding-top:12px;
            
            
        }
        .text-wrapper{
            display:flex; 
            align-items:center;
        }
        ::slotted([slot=children]) {
            margin-top: 8px;
            margin-left:16px;
            
        }
        img {
            margin: 0 8px;
        }
        
    
        ul{
            margin:0;
        }
        p{
            line-height:0;
            margin:0;
        }
    
  
    `,
    ];
  }

  @property()
  label: string = "";

  @property()
  path: string = "";

  @property({ type: Boolean })
  open: boolean = false;

  render() {
    const { label} = this;

    return html`
      <div class="main-wrapper">
        <div class="inside-wrapper">
          <div class="text-wrapper flex py-3">
            <p>${label}</p>
          </div>
        </div>
        <ul>
          <slot></slot>
        </ul>
      </div>

        
    `;
  }
}
