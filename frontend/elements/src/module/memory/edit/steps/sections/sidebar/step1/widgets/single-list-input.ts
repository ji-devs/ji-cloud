import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

@customElement('sidebar-widget-single-list-input')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
                width: 492px;
              display: flex;
              justify-content: center;
              width: 100%;
          }
        input {
            width: 460px; /*arbitrary amount to not go into rounded corners*/
          outline: none;
          border: none;
          font-size: 16px;
          text-align: center; 
        }

        :host([placeholder]) input {
            color: var(--light-gray-4); 
        }

    `];
  }

  @property()
  value:string = "";

  @property({type: Boolean, reflect: true})
  placeholder:boolean = false;

  render() {
      const {value} = this;

      return html`<div class="row">
          <input type="text" value=${value}></input>
      </div>`
  }
}
