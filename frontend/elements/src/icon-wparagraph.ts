import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('icon-wparagraph')
export class _ extends LitElement {
  static get styles() {
    return [css`
  img-ui{
    width: 246px;
    height: 236px;
    object-fit: contain;

  }
  h2{
    font-family: Poppins;
    font-size: 18px;
    font-weight: bold;

  }
  p{
    font-family: Poppins;
    font-size: 16px;
    line-height: 1.5;
    color: #383838;
    margin-top:5px;

  }
  .wrapper{
    width: 274px;
margin-left:50px;

  }
  .pink {
    color: #fd6b71;
  }
  .darkblue{
    color: #2040a3;
  }
  .green{
    color: #46ba6f;
  }
  .orange{
    color: #fea559;
  }
  .lightblue{
    color:#6ca1fc;
  }

  .inside{
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: 1fr 1fr 1fr;
  }
    `]
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
     <div class="wrapper">
        <img-ui class="img" path="${path}"></img-ui>
        <div class="inside">
          <h2 class="${color}">${title}</h2>
          <p>${paragraph}</p>
          <slot></slot>
        </div>
      </div>
        
  `;
  }
}

 