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
            cursor:pointer;
        }
        .circle{
            width: 6px;
            height: 6px;
            border-radius:50%;
            background-color: #83aef7;
            margin-right:4px;
        }
       
     
       
    `];
  }


  render() {
   
 
    return html`
<div class="wrapper">
       <div class="circle"></div>
       <div class="circle"></div>
       <div class="circle"></div>
       
</div>
  `;
  }
}