import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('reccomends-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;

    }
   main{
    width: 1920px;
     height: 287px;
    background-color:#d8e7fa;
   }
   
   img-ui{
margin-top:80px;
margin-left:48px;
margin-right:48px;
   }
   .fliparrow{
    transform: scaleX(-1);

   }

 
    `];
  }
  @property()
  PATH_ARROW:string = ""; 




  render() {

    const {} = this;
    const PATH_ARROW="icn-arrow.svg"



    return html`
    <main>
     <div class="inside-wrapper">
     <img-ui class="img" path="${PATH_ARROW}"></img-ui>

        <slot name="icon-title-paragraph"></slot>
    <img-ui class="fliparrow" path="${PATH_ARROW}"></img-ui>

    </div>
    </main>
  `;
  }
}