import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/variants/horizontal-underlined-title";
@customElement('image-menu')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .list-object li{
        padding-left:16px;
        cursor:pointer;
    }

    .list-object li::before{
        content:'';
        height:16px;
        width:16px;
        border-radius:50%;
        display:inline-block;
        margin-right:8px;
    }
    .list-object li{
      padding-left:16px;
  }

  .published::before{
      background-color: #6eca90;
      content:'';
      height:16px;
      width:16px;
      border-radius:50%;
      display:inline-block;
      margin-right:8px;
  }
  .saved::before{
     background-color: #e36486;
     content:'';
     height:16px;
     width:16px;
     border-radius:50%;
     display:inline-block;
     margin-right:8px;

  }
 
    `];
  }
  

  render() {
      
   
    const STR_PUBLISHED ="Show published";
    const STR_SAVED = "Show saved";
    const STR_SEEALL ="See all"

    return html`
 <div class="list-object">
                <li class="">${STR_SEEALL}</li>
                </div>
                <div class="list-object">
                <li class="published">${STR_PUBLISHED}</li>
                </div>
                <div class="list-object">
                <li class="saved">${STR_SAVED}</li>
                </div>
                
           
  `;
  }
}