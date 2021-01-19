import { LitElement, html, css, customElement, property} from 'lit-element';
import {nothing} from "lit-html";
import {colorStyles} from "@elements/_styles/colors";
import {arrayCount} from "@utils/array";

@customElement('steps-nav')
export class _ extends LitElement {

  static get styles() {
    return [colorStyles, css`
      main {
        border-color: rgba(226,229,235,1);
        border-bottom-width: 1px;
      }
      .steps {
        display: flex;
        align-items: center;
        justify-content: space-between;
        font-size: 14px;
        padding-left: 16px;
        padding-right: 16px;
        padding-bottom: 16px;
        padding-top: 32px;
      }
      .separator {
        background-color: rgba(161,168,173,1);
        height: 1px;
        margin-top: -15px;
        width: 41px;
      }
    `];
  }

  @property({type: Number})
  count:number = 0;

  // Define the element's template
  render() {
    const {count} = this;

    return html`
      <main>
        <div class="steps">
          ${arrayCount(count)
              .map(step => makeStep(step, count))
          }
        </div>
      </main>
    `;
  }
}

const makeStep = (step:number, lastStep:number) => html`
  <slot name="slot-${step}"></slot>
  ${step !== lastStep 
      ? html`<div class="separator"></div>` 
      : nothing
   }
`