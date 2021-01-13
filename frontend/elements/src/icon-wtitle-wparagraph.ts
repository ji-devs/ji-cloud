import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('icon-wtitle-wparagraph')
export class _ extends LitElement {
  static get styles() {
    return [css`
  img-ui{
    width: 246px;
    height: 236px;
    object-fit: contain;

  }
  h2{
    font-size: 32px;
    font-weight: 900;
    margin-top:50px;

  }
  p{
    font-family: Poppins;
    line-height: 1.5;
    color: #383838;
    margin-top:10px;

  }
  div{
    width: 274px;
margin-left:50px;
  }
  }
    `];
  }


  @property()
  path:string = ""; 
  @property()
  paragraph:string = ""; 
  @property()
  title:string = ""; 
  @property()
  color:string = ""; 
 

  render() {

    const {path, paragraph,title,color} = this;

    return html`
     <div>
        <img-ui class="img" path="${path}"></img-ui>
        <h2>${title}</h2>
        <p>${paragraph}</p>
        <slot></slot>
        </div>
        // style="color:"${color}"
  `;
  }
}