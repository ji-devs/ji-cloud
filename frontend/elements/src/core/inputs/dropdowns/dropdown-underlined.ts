import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
@customElement("dropdown-underlined")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
      .wrapper {
        margin-bottom: 16px;
        position:relative;
        cursor:pointer;
      }
   
      span {
        color: #5590fc;
        margin-bottom: 8px;
      }
      .input-wrapper {
        display: flex;
        align-items: center;
        border-bottom: solid 1px #e5e7ef;
        position: relative;
        padding-bottom:8px;
      }

      input {
        outline: none;
        border: none;
        font-size: 16px;
        padding: 0 8px;
        width: 100%;
      }
      focus {
        outline: none;
      }
      ::placeholder {
        color: #a1a8ad;
      }
      img-ui {
        position: absolute;
        right: -10px;
        cursor:pointer
      }
      .closed img-ui{
          transform:rotate(90deg)
      }
      .open img-ui{
        transform:rotate(-90deg)
    }
    .dropdown{
        padding:8px 0;
        background-color:#ffffff;
        box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
        border: solid 1px #f0f1f5;    
        position: absolute;  
        z-index:10;
        width:100%;
      }
    ul{
        padding:0;
        list-style-type: none;
    }
    ::slotted(*:hover){
        background-color: #e6f0ff;
      
      }
      .closed .dropdown{
          display:none;
      }
      
   
      `,
    ];
  }

 
  @property({type:Boolean})
  closed: boolean = false;

  render() {
    const {closed } = this; 
    const STR_SEEALL = "See all"
    return html`
         <div class="wrapper ${closed ? "closed" : "open"}">
      
          <div class="input-wrapper">
            <input
              class=""
              type="text"
              placeholder="${STR_SEEALL}"
              aria-label="Full name"
              value="${STR_SEEALL}"
              readonly
            />
            <img-ui path="icon-chevron-categories-24-px.svg"></img-ui>
          </div>
       
        <div class="dropdown">
        <ul>
        <div class="slot-wrapper">
        <slot></slot>
        </div>
        </ul>
        </div>
      </div>

    `;
  }
}
