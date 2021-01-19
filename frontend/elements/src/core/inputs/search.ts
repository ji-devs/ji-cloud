import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';

@customElement('input-search')
export class _ extends LitElement {

  static get styles() {
    return [css`
   .wrapper{
    position:relative;
    width:200px;
    height:32px;
   
   }
   input{
        border:none;
        width:200px;
        height:32px;
        padding: 4px 40px 4px 16px;
        border-radius: 18px;
        border: solid 1px #e5e7ef;
        background-color: #f8f9fd;
        position:absolute;
        font-size:16px;

   }
   input:focus{
       outline:none;
   }
   img-ui{
       position:absolute;
       right: 10px;
       z-index:10;
       top:5px;
       

   }
   input[type="search"]::-webkit-search-decoration,
    input[type="search"]::-webkit-search-cancel-button,
    input[type="search"]::-webkit-search-results-button,
    input[type="search"]::-webkit-search-results-decoration { display: none; }
    `];
  }

  @property()
  label: string = "";

  @property()
  value: string = "";

  render() {

    const {label, value} = this;
    return html`
    <div class="wrapper">
        <img-ui path="REPLACE-ME.svg" alt="" class=""></img-ui>
        <input  type="search" name="" value="${value}" placeholder="${label}">
    </div>
  

  `;
  }
}