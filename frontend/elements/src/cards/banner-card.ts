import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('banner-card')
export class _ extends LitElement {

  static get styles() {
    return [css`
    .wrapper{
        width: 354px;
        height: 40px;
        box-shadow: 2px 3px 2px 0 rgba(208, 211, 232, 0.5);
    }
  
    `];
  }

@property()
path: string = "";

@property()
label: string = "";

  render() {
    const {path, label} = this;
    return html`
<div class="wrapper">
    <span>
        <img-ui path="${path}"></img-ui>
        <p>${label}</p>
    </span>
</div>
  `;
  }
}