import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

@customElement('sidebar-widget-dual-list-input')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              display: flex;
              justify-content: center;
              width: 100%;
          }
        textarea {
          outline: none;
          border: none;
          font-size: 16px;
          text-align: center; 
          resize: none;
        }

        :host([placeholder]) textarea {
            color: var(--light-gray-4); 
        }

    `];
  }

  onInput(evt:InputEvent) {
    const {value} = (evt.target as any);
    this.value = value;

    this.dispatchEvent(new CustomEvent("custom-input", {
      detail: { value },
    }))
  }
  onChange(evt:InputEvent) {
    const {value} = (evt.target as any);
    this.value = value;

    this.dispatchEvent(new CustomEvent("custom-change", {
      detail: { value },
    }))
  }
  @property()
  value:string = "";

  @property({type: Boolean, reflect: true})
  placeholder:boolean = false;

  render() {
      const {value} = this;

      return html`<div class="row">
          <textarea @input="${this.onInput}" @change="${this.onChange}" .value="${value}" ></textarea>
      </div>`
  }
}
