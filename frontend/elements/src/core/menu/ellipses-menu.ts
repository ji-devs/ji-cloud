import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('ellipses-menu')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
        display:relative;
    }

    
    `];
  }

  @property({type:Boolean})
  clicked:boolean = true;

  render() {
      const {clicked} = this;
    return html`
    <div class="wrapper ${clicked ? "clicked" : "hide"}">
    <button-ellipses></button-ellipses>
    <div class="dropdown">
       <slot></slot>
    
       </div>
    </div>
  
  `;
  }
}