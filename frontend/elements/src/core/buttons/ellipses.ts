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
       
        .dropdown{
            width: 125px;
            height: 148px;
            border-radius: 4px;
            box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.25);
            padding:16px;
            display:none;
            background-color:#ffffff

        }
        .clicked{
            display:block;
            z-index:2;
        }
       
    `];
  }

 @property({type:Boolean})
clicked:boolean = true;

  render() {
   const {clicked} = this;
  
 
    return html`
<div class="wrapper">
       <div class="circle"></div>
       <div class="circle"></div>
       <div class="circle"></div>
       <div class="dropdown ${clicked ? "clicked" : ""}">
       <slot></slot>
    
       </div>
</div>
  `;
  }
}