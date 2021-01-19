import "@elements/buttons/circle-button";

import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property} from 'lit-element';
import {nothing, TemplateResult} from "lit-html";
import {colorStyles} from "@elements/_styles/colors";
import {arrayIndex} from "@utils/array";

@customElement('legacy-sidebar')
export class _ extends LitElement {

  static get styles() {
    return [colorStyles, css`
        aside {
            overflow-y: auto;
            height: 100%;
            width: 350px;
            display: flex;
            flex-direction: column;
        }

        aside.small {
          width: auto;
        }

        .modules {
          display: flex;
          flex-direction: column;
          align-items: center;
        }

        .close {
          align-self: flex-end;
          cursor: pointer;
        }

        .open {
          position: absolute;
          cursor: pointer;
        }

    `];
  }

  @property({type: Number})
  nModules:number = 0;

  @property({type: Boolean})
  closed:boolean = false;

  // Define the element's template
  render():TemplateResult {
    const {nModules, closed} = this;

    return closed ? makeClosed(this) : makeOpen(this, nModules);
  }
}

function makeClosed(dispatcher:LitElement) {
    return html`
        <div class="open" @click="${() => dispatcher.dispatchEvent(new Event("open"))}">
        -->
        </div>
    `
}

function makeOpen(dispatcher:LitElement, nModules:number) {
    return html`
      <aside>
        <div class="close" @click="${() => dispatcher.dispatchEvent(new Event("close"))}">
          <-- close
        </div>
        <div class="modules">
          ${arrayIndex(nModules)
              .map(index => moduleSlot(index, nModules))
          }
        </div>
      </aside>
    `;
}

const moduleSlot = (index:number, max:number) => html`
    <slot name="module-${index}"></slot>
    ${index !== max
        ? html`<div class="separator"></div>` 
        : nothing
    }
`