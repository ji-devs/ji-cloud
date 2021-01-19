import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('jiggling-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;
    }

    
   main{
    background-color:#fd6b71;
    width: 1920px;
    height: 783px;
   }
   
   ::slotted([slot=title]){
    display:block
    text-align: center;
   }

   .right-side{
       margin-left:1011px;
   }
 
   .4points{
    display:flex;

   }
    `];
  }



  render() {

    const {} = this;

    return html`
    <main>
    <div class="inside-wrapper">
    <slot name="title"></slot>
    </div>

    <div class="right-side">
        <slot name="title-sub-paragraph"></slot>
      
        <div class="4points">
        <slot name="points"></slot>
        </div>
        </div>
        <div class="left-side">
        <slot name="title-sub-paragraph"></slot>
      
        <div class="4points">
        <slot name="points"></slot>
        </div>
        </div>


    </main>
  `;
  }
}