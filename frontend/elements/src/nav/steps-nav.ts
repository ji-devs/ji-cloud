import "@elements/buttons/circle-button";

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
  steps:number = 0;

  // Define the element's template
  render() {
    const {steps} = this;

    return html`
      <main>
        <div class="steps">
          ${arrayCount(steps)
              .map(step => makeStep(step, steps))
          }
        </div>
      </main>
    `;
  }
}

const makeStep = (step:number, lastStep:number) => html`
  <slot name="btn-${step}"></slot>
  ${step !== lastStep 
      ? html`<div class="separator"></div>` 
      : nothing
   }
`