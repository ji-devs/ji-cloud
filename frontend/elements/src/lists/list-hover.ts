import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('list-hover')
export class _ extends LitElement {
  static get styles() {
    return [css`
    ul{
        padding:0;
        margin:0;
        max-height:400px;
        overflow:auto
    }
  li{
      margin-bottom:4px;
      list-style-type: none;
      display:flex;
      justify-content:space-between
      
  }
  li:hover{
      background-color: #e7f0fe;
      
  }
  p{
    padding-left:20px;
    padding-right:20px;
    margin:0;
  }
  img-ui{
    display:none;
  }
  .checked{
    display:block;
    padding-right:20px;
  }
    `];
  }

  @property({type:Boolean})
  checked: boolean = false;
 

  render() {

    const {checked} = this;

    return html`
    
    <ul>

        <li>
        <p> All languages</p>
           <img-ui class="${checked ? 'checked' : ''}" path="icn-chosen-check.svg"></img-ui>
        </li>
        <li><p>English</p></li>
    </ul>
  `;
  }
}