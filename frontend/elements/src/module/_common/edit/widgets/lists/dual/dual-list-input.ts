import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import { live } from "lit-html/directives/live";

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


  @property()
  constrain:((text:string) => string) | undefined = undefined;

  setValue = (value:string) => {
    const {constrain} = this;
    this.value = constrain ? constrain(value) : value;
  }

  onInput(evt:InputEvent) {
    this.setValue((evt.target as any).value);
    this.dispatchEvent(new CustomEvent("custom-input", {
      detail: { value: this.value },
      }))
  }
  onChange(evt:InputEvent) {
    this.setValue((evt.target as any).value);
    this.dispatchEvent(new CustomEvent("custom-change", {
      detail: { value: this.value },
      }))
  }

  @property({hasChanged: () => true})
  value:string = "";

  @property({type: Boolean, reflect: true})
  placeholder:boolean = false;

  @property({type: Number})
  rows:number = 1;

  render() {
      const {value, rows} = this;

      return html`<div class="row">
          <textarea @input="${this.onInput}" @change="${this.onChange}" .value="${live(value)}" rows="${rows}"></textarea>
      </div>`
  }
}
