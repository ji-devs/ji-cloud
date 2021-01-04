import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('password-strength')
export class _ extends LitElement {
  static get styles() {
    return [css`
  .wrapper {
    width: 257px;
    height: 8px;
    border-radius: 4px;
    position:relative;
    margin-bottom: 12px;
  }
  .inner{
      position:absolute;
      border-radius: 4px;
      height: 8px;
      background-color: #42cc7a;
      width: 75%;
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