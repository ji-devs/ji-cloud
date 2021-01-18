import { MEDIA_UI } from '@utils/path';
import "@elements/column-list"
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('footer-kidsafe')
export class _ extends LitElement {
  static get styles() {
    return [css`
  img-ui{
    width: 156px;
  height: 54px;
     object-fit: contain;
   nargin-right:23px;

  }
  .wrapper{
//     width: 274px;
// margin-left:50px;
display:flex;
  }
#stripe{
    width:1px;
    height: 20px;
    background-color: #ffffff; 
    opacity: 0.35;
    margin-left:16px;
    margin-right:16px;

}
    `]
  }
  @property()
  path_kidsafe:string = ""; 
  @property()
  term:string = ""; 
  @property()
  privacy:string = ""; 
  render() {
    const {path_kidsafe, term,privacy } = this;
    return html`
     <div class="wrapper">
        <img-ui class="img" path="${path_kidsafe}"></img-ui>
        <column-list text_line="${term}" ></column-list>
        <div class="stripe"></div>
        <column-list text_line="${privacy}" ></column-list>
      </div>
  `;
  }
}