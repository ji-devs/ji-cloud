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
    border: solid 1px #b0ccf2;
    background-color: #f3f8fe;
    display:flex;
    justify-content:center;
    align-items:center;
    font-size:14px;
    margin-right:8px;
    height:24px;
    color:#387af4;
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
.negative{
  border: solid 1px #6ea3f9;
  color:#afcbf4;

}
    `];
  }



  @property()
  label:string = ""; 

  @property({type:Boolean})
  negative:boolean = false; 

  @property()
  path:string = "icn-delete-tab.svg"; 

  render() {

    const {label,path, negative} = this;

    return html`
<div class="wrapper ${negative ? 'negative' : ''}">${label}
<img-ui path="${path}"></img-ui>
</div>

  `;
  }
}