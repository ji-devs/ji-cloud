import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';

@customElement('icon-button')
export class _ extends LitElement {

  static get styles() {
    return [css`
    button {
        border-radius: 24px;
        border: none;
        cursor: pointer;
        font-size: 16px;
        
      }
      .medium{
          padding: 13.6px 24px 11.4px;
      }
      .large{
       padding: 15px 40px 16px;
      }
      
      .red {
        background:#fd6b71;
        color:#fff;
      }
      
      .red:hover{
        background: #ed6065
      }
      
      .blue{
        background:#5590fc;
        color:#ffffff;
      }
      


      
      .blue:hover{
        background: #387af4
      }
      

      
      button:disabled{
        background: #a9b1b5;
        color: #e7edf0
      }
      button:focus{
          outline:none;
      }
      .bold {
        font-weight: bold;
      }
      .green{
        background-color: #71cf92;
        color:#ffffff;
      }
      .green:hover{
        background-color: #46ba6f;
      }
      .bolder{
          font-weight:600;
      }
    
    `];
  }

  @property()
  size: string = "";
  @property()
  label: string = "";
  @property()
  color: string = "";
  @property()
  fontweight: string = "";

  render() {

    const {size, label, color, fontweight} = this;

    return html`
      <button type="button" name="button" class="${size} ${color} ${fontweight}" >
      
      ${label}
    </button>
  `;
  }
}