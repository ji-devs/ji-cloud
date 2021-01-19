
  import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('placeholder-img')
export class _ extends LitElement {
  static get styles() {
    return [css`
  .wrapper {
    width: 362px;
        height: 362px;
        background-color: #f8f9fd;
        border-radius:50%;
        display:block;
  }

   
    `];
  }


  
  render() {

    const {} = this;

    return html`
   
        <div class="wrapper">
          <div class="inner"></div>

        </div>
      

  `;
  }
}