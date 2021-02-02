import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';

const STR_LABEL = "Sign in with Google";


@customElement('button-google')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
        
        border-radius: 14px;
        box-shadow: 2px 2px 3px 0 rgba(0, 0, 0, 0.16);
        border: solid 1px #f0f1f4;
        background-color: #ffffff;
        height:64px;
        width: 296px;
        align-items:center;
        cursor:pointer;
    }
    span{
        display:flex;
        align-items:center;
        font-size:22px;
    }
    
   
    `];
  }

  render() {

    return html`
   
        <div class="wrapper">
          <span class="flex items-center font-sans font-normal text-lg">
            <img-ui path="icn-google-button-64.svg" alt=""></img-ui>
            ${STR_LABEL}
          </span>

        </div>
      

  `;
  }
}