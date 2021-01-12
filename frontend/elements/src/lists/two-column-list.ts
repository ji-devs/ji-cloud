import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('twocolumn-list')
export class _ extends LitElement {
  static get styles() {
    return [css`
        .wrapper{
            display:flex;
            
        }
    
    `];
  }

  render() {

    const {} = this;

    return html`
    <div class="wrapper">
       <slot name="left"></slot>
       <slot name="right"></slot>
    </div>
  `;
  }
}