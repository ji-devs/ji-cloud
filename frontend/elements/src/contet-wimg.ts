import { MEDIA_UI } from '@utils/path';
import "@elements/column-list"
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('content-wimg')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
        display:flex;
        width: 1676px;
        height: 540px;
        background-color:#ffffff ;
        margin-left:122px;
        border-radius: 25px;
        box-shadow: 0 3px 20px 0 rgba(0, 0, 0, 0.16);
 
 
     }

     ::slotted([slot=title]){
        font-size: 64px;
        font-weight: 900;
        color:#5662a3;
        text-align: center;
       }
    
      img-ui{
        width: 960px;
        height: 540px;

       }
    
       ::slotted([slot=content]){
        width: 716px;
        height: 540px;
    }
    
    ::slotted([slot=subtitle]){
      margin-right:107px;
      margin-top:56px;
      // margin-left:1017px;
    
     }

     .lines{
        width: 604px;
        height: 119px;
        margin-right:56px;
        margin-top:144px;
    }


  
    `]
  }

  @property()
  pathimg:string = ""; 
 
 
 

  render() {

    const {pathimg} = this;

    return html`
    <div class="inside-wrapper">
    <slot name="img"></slot>
    <img-ui path="${pathimg}"></img-ui>
     <slot name="content">
     <slot name="subtitle"></slot>
     <div class="lines">
     <slot name=line></slot>
     </div>
     <slot name="button"></slot>
     </slot>
    </div>
        
  `;
  }
}