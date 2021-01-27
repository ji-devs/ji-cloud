import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('title-w-icon')
export class _ extends LitElement {
  static get styles() {
    return [css`
        .wrapper{
            display: flex;
            align-items:center;
            position:relative;
        }
        .uploaded{
            color: #46ba6f;
        }
        p{
            font-size: 13px;
            font-weight: 500;
        }
        img-ui{
          margin-right: 12px
        }
        .delete{
          display:none;
        }
        .wrapper:hover .delete{
          display:block;
        }
        .wrapper:hover p{
          color:#5590fc;
        }
       
       

   
    `];
  }

  @property()
  title:string = ""; 
  @property()
  path:string = ""; 
  @property({type: Boolean})
  uploaded: boolean = false;

  @property({type: Boolean})
  hover:boolean = false;
  
  enter() {
    this.hover = true
}
leave() {
    this.hover = false
}
  
  render() {

    const {title, path, uploaded,hover} = this;

    const src = hover ? "icn-delete.svg" : "icn-delete-blue.svg";
    return html`
    <div class="wrapper" @mouseenter="${this.enter}" @mouseleave="${this.leave}">
        <img-ui path="${path}"></img-ui>
        <p class="${uploaded ? 'uploaded' : ''}">${title}</p>
        <img-ui path="${src}" class="delete"></img-ui>
        
    </div>
  `;
  }
}