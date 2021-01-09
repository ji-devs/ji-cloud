import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('pill-listitem')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
    width: 88px;
    padding: 3px 0;
    border-radius: 12px;
    border: solid 1px #5590fc;
    background-color: #c4dbff;
    display:flex;
    justify-content:center;
    align-items:center;
    font-size:14px;
    margin-top:20px;
    margin-right:8px;
}
img-ui{
  position:absolute;
  top: -7px;
  left: 80px;
  display:none;
 
}

.wrapper:hover img-ui{
  display:block;
}
    `];
  }



  @property()
  label:string = ""; 

  @property()
  path:string = "icn-delete-tab.svg"; 

  render() {

    const {label,path} = this;

    return html`
<div class="wrapper">${label}
<img-ui path="${path}"></img-ui>
</div>

  `;
  }
}