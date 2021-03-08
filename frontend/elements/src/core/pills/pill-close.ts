import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('pill-close')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
    width: 88px;
    
    border-radius: 12px;
    border: solid 1px #b0ccf2;
    background-color: #f3f8fe;
    display:flex;
    justify-content:center;
    align-items:center;
    font-size:14px;
    margin-right:8px;
    height:24px;
    color:#387af4;
    position:relative;
}

img-ui{
  position:absolute;
  top: -7px;
  left: 80px;
  display:none;
  height:16px;
  width:16px;
 
}

.wrapper:hover img-ui{
  display:block;
  cursor:pointer;
}
.negative{
  border: solid 1px #6ea3f9;
  color:#afcbf4;
  background-color:inherit;

}
    `];
  }
  @property({type:Boolean})
  negative:boolean = false; 

  @property()
  label:string = ""; 

  render() {

    const {negative, label} = this;

    return html`
      <div class="wrapper ${negative ? 'negative' : ''}">
        <p>${label}</p>
        <img-ui path="icn-delete-tab.svg"></img-ui>
      </div>

  `;
  }
}