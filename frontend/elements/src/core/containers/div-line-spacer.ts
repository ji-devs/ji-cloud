import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import {BaseButton} from "@elements/_styles/buttons";
import { arrayCount } from '@utils/array';
import { nothing } from 'lit-html';

//TODO - don't require nSlots: https://stackblitz.com/edit/lit-tabs-with-dynamic-slots
//(a bit complicated with MutationObserver but it's abstracted)

@customElement('div-line-spacer')
export class _ extends BaseButton {

  static get styles() {
    return [css`
    .wrapper{
        display:flex;
        align-items:center;
        height:20px;
        
    }
      p{
        font-weight: 500;
        cursor: pointer;
      }
      .none{
        padding: 0;
      
      }
      .medium{
          padding: 13.6px 24px 11.4px;
      }
      .large{
       padding: 15px 40px 16px;
      }
      
      .red {
        color:#fd6b71;
      }
      
      .red:hover{
        color: #ed6065
      }
      
      .blue{
        color:#5590fc;
      }
      
      .blue:hover{
        color: #387af4
      }
      
      button:disabled{
        color: #a9b1b5;
        
      }
  
      .bold {
        font-weight: bold;
      }
      .green{
        color: #71cf92;
        
      }
      .green:hover{
        color: #46ba6f;
      }
      .spacer{
        width: 2px;
        height: 20px;
        background-color:#5590fc;
        margin:0 16px;
      }
  
    
    `];
  }

  @property({type: Number})
  nSlots: number = 0; 

  render() {
    const {nSlots} = this;

    return html`
      <div class="wrapper">
        ${arrayCount(nSlots).map(i => {
            return html`
              <slot name="slot-${i}"></slot>
              ${i !== nSlots
                 ? html`<div class="spacer"></div>`
                 : nothing
              }
            `
        })}
      </div>
  `;
  }
}