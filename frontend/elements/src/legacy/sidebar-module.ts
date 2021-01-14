import "@elements/buttons/circle-button";

import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property} from 'lit-element';
import {nothing} from "lit-html";
import {colorStyles} from "@elements/_styles/colors";
import {arrayIndex} from "@utils/array";

@customElement('legacy-sidebar-module')
export class _ extends LitElement {

  static get styles() {
    return [colorStyles, css`
        section {
            display: flex;
            flex-direction: column;
        }

        .caption {
            text-align: center;
            width: 100%;
        }
    `];
  }

  @property()
  jigId:string = ""

  @property()
  moduleId:string = ""

  @property({type: Number})
  index:number = 0

  // Define the element's template
  render() {
    const {img, index} = this;

    return html`
      <section>
        <div>
            <slot name="img"></slot>
        </div>
        <div class="caption">${index}</div>
      </section>
    `;
  }
}