import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/variants/horizontal-underlined-title";

@customElement('search-pagination')
export class _ extends LitElement {
  static get styles() {
    return [css`
   main{
     display:flex;
   }
   .wrapper{
     display:flex;
   }
   .inner-wrapper{display:flex;}
   .left-arrow{
     transform:rotate(180deg);
     display:flex;
     margin-top:-2px;
     
   }
   p{
     margin:0;
   }
   .page-number{
    width: 24px;
    height: 24px;
    border: solid 1px #c4dbfe;
    background-color: #e6f0ff;
    display:flex;
    align-items:center;
    text-align:center;
    color: #5590fc;
    margin:0 8px;

   }
   input:focus{
     outline:none;
   }
   input[type=number]::-webkit-inner-spin-button, 
input[type=number]::-webkit-outer-spin-button { 
  -webkit-appearance: none; 
  margin: 0; 
}
.arrow{
  display:flex;
  margin-top:-6px;
}
.back{
  margin-right:55px;
  cursor:pointer;
}
.next{
  margin-left:55px;
  cursor:pointer;
}
span{
  margin-left: 4px;
}

 
    `];
  }

  @property()
  number:string =  ""

  render() {
    const {number} = this;
    const STR_BACK = "Back";
    const STR_PAGE = "Page";
    const STR_OF = "of";
    const STR_NEXT ="Next";
    return html`
    
    <main>
      <div class="wrapper">
        <div class="inner-wrapper back">
          <img-ui path="icon-chevron-categories-24-px.svg" alt="" class="left-arrow"></img-ui>
          <p>
            ${STR_BACK}
          </p>
        </div>
        <div class="inner-wrapper">${STR_PAGE}
          <input value="1" type="number"
            class="page-number">
    
          ${STR_OF}&nbsp;
          <slot></slot>
        </div>
        <div class="inner-wrapper next">
          <p>${STR_NEXT}</p>
          <img-ui path="icon-chevron-categories-24-px.svg" alt="" class="arrow"></img-ui>
        </div>
      </div>
    </main>
  `;
  }
}