import { MEDIA_UI } from '@utils/path';
import "@elements/entry/home/TOSORT/column-list";
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
    margin-bottom:99px;
    display: block;
    margin-left:123px;

  }
  .wrapper{
//  width: 274px;
// margin-left:50px;
display:flex;
  }
.stripe{
    width:1px;
    height: 20px;
    background-color: #ffffff; 
    opacity: 0.35;
    margin-left:16px;
    margin-right:16px;
    display: block;

}
column-list{
  list-style-type: none;
  margin-left:16px;
  display: block;
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
    const STR_WHITE="white"
    return html`
     <div class="wrapper">
        <img-ui class="img" path="${path_kidsafe}"></img-ui>
        <column-list text_line="${term}" color="${STR_WHITE}" ></column-list>
        <div class="stripe"></div>
        <column-list text_line="${privacy}" color="${STR_WHITE}"></column-list>
      </div>
  `;
  }
}