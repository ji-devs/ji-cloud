import "@elements/buttons/circle-button";

import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property} from 'lit-element';
import {nothing} from "lit-html";
import {colorStyles} from "@elements/_styles/colors";
import {arrayIndex} from "@utils/array";

@customElement('legacy-sidebar')
export class _ extends LitElement {

  static get styles() {
    return [colorStyles, css`
        aside {
            overflow-y: auto;
            height: 100%;
        }

        .module:hover {
            cursor: pointer; 
        }
    `];
  }

  @property({type: Number})
  nModules:number = 0;

  // Define the element's template
  render() {
    const {nModules} = this;

    return html`
      <aside>
        <div class="modules">
          ${arrayIndex(nModules)
              .map(index => moduleSlot(index, nModules))
          }
        </div>
      </aside>
    `;
  }
}

const moduleSlot = (index:number, max:number) => html`
    <div class="module">
        <slot name="module-${index}"></slot>
    </div>
    ${index !== max
        ? html`<div class="separator"></div>` 
        : nothing
    }
`