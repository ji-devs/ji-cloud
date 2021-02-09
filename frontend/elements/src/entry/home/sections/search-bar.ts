import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';

@customElement('search-bar')
export class _ extends LitElement {

  static get styles() {
    return [css`
    .wrapper{
        width: 976px;
        height:52px;
        border-radius:36px;
        display:flex;
        background-color:#ffffff;
        align-items:center;
        position:relative;
    }
    input{
        border:none;
        font-size: 20px;
        width:410px;
        margin-left:24px;
    }
    input::placeholder{
        color: #a9b1b5;
    }
   input:focus{
       outline:none;
   }

   input[type="search"]::-webkit-search-decoration,
    input[type="search"]::-webkit-search-cancel-button,
    input[type="search"]::-webkit-search-results-button,
    input[type="search"]::-webkit-search-results-decoration { display: none; }

    .age{
      width:164px;
      border-left:solid 1px #a9b1b5;
      border-right:solid 1px #a9b1b5;
      height:24px;
      padding:0 16px
     

   }
   .language{
    height:24px;
    padding:0 16px;
   }
   ::slotted([slot="advanced"]){
    position:absolute;
    text-align:center;
    right:-100px;
    
  }
  
    `];
  }

  @property()
  label: string = "";

 

  render() {

    const {label} = this;
    const STR_LOOIKINGFOR = "What are you looking for?"
    return html`
    <div class="wrapper">
        <input  type="search" name="" value="" placeholder="${STR_LOOIKINGFOR}">
        <div class="age">
          <slot name="dropdown"></slot>
        </div>
        <div class="language">
          <slot name="dropdown-language"></slot>
        </div>
        <slot name="button"></slot>
        <slot name="advanced"></slot>
    </div>
  `;
  }
}