import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('button-ellipses')
export class _ extends LitElement {

  static get styles() {
    return [css`
        .wrapper{
            display:flex;
            padding-top:8px;
            margin-left:48px;
            height:6px;
            position:relative;
            cursor:pointer;
        }
        .circle{
            width: 6px;
            height: 6px;
            border-radius:50%;
            background-color: #83aef7;
            margin-right:4px;
        }
        ul{
            list-style-type:none;
            padding:0;
        }
        .dropdown{
            width: 125px;
            height: 148px;
            border-radius: 4px;
            box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.25);
            padding:16px;
            display:none;
        }
        .clicked{
            display:block;
        }
        li{
            cursor:pointer;
        }
    `];
  }

 @property({type:Boolean})
clicked:boolean = false;

  render() {
   const {clicked} = this;
   const STR_SIBLING = "Add a sibling";
   const STR_CHILD ="Add a child";
   const STR_RENAME = "Rename";
   const STR_DELETE = "Delete";
   const STR_HIDE = "Hide";
 
    return html`
<div class="wrapper">
       <div class="circle"></div>
       <div class="circle"></div>
       <div class="circle"></div>
       <div class="dropdown ${clicked ? "clicked" : ""}">
       <ul>
       <li>${STR_SIBLING}</li>
       <li>${STR_CHILD}</li>
       <li>${STR_RENAME}</li>
       <li>${STR_DELETE}</li>
       <li>${STR_HIDE}</li>
       </ul>
       </div>
</div>
  `;
  }
}