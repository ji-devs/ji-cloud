import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('tooltip-popup')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
      width: 326px;
      height: 80px;
      border-radius: 12px;
      border: solid 1px #6eca90;
      background-color: #ffffff;
      display:flex;
      align-items:center;
      position:absolute;
      padding: 12px 16px;
      top:45px;
      right:33%
     
      

    }
    .thumbnail{
      width: 76px;
    height: 56px;
    }
    .icon{
      margin-left:24px;
      margin-right:8px;
    }
    p{
      color:#6eca90;
    }


    `];
  }



  @property()
  label:string = ""; 

  @property()
  path:string = ""; 

  @property()
  icon:string = ""; 

  render() {

    const {path,label, icon} = this;

    return html`
 
 
        <div class="wrapper rounded-md p-2  z-10 absolute top-50 left-50p">
          <img-ui class="thumbnail" path="${path}" ></img-ui>
          <img-ui class="icon" path="${icon}"></img-ui>
          <p>${label}</p>
        </div>
    
</div>
  `;
  }
}