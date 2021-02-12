import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('button-ellipses')
export class _ extends LitElement {

  static get styles() {
    return [css`
        :host {
            display: grid;
            grid-template-columns: 6px 6px 6px;
            grid-template-rows: 6px 6px 6px;
            gap: 0px 4px;
            grid-template-areas:
                ". . ."
                "l m r"
                ". . .";

            cursor:pointer;
        }
        .circle{
            width: 6px;
            height: 6px;
            border-radius:50%;
            background-color: #83aef7;
        }
       
     
       
    `];
  }


  render() {
   
 
    return html`
       <div class="circle" style="grid-area: l"></div>
       <div class="circle" style="grid-area: m"></div>
       <div class="circle" style="grid-area: r"></div>
  `;
  }
}