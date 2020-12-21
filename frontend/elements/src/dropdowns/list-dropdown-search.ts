import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('list-dropdown')
export class _ extends LitElement {
  static get styles() {
    return [css`
   
    `];
  }

  @property()
  path:string = ""; 

  render() {

    const {path} = this;

    return html`
        <div class="wrapper">
        <!--This should be toggled when clicking on filter FILTERBOX-->
        <div class="dropdown">
            <slot name="one"></slot>
            <slot name="two"></slot>
            

           




        </div>

        </div>
  `;
  }
}