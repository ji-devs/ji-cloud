import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('ellipses-menu')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .dropdown.menuVisible{display:block}
    .dropdown{
      display:none;
    }
    
    

    
    `];
  }

  @property({type:Boolean})
  menuVisible:boolean = false;

  render() {
      const {menuVisible} = this;
    return html`
    <div class="wrapper ">
    <button-ellipses></button-ellipses>
    <div class="dropdown ${menuVisible ? "menuVisible" : ""}">
       <slot></slot>
    
       </div>
    </div>
  
  `;
  }
}