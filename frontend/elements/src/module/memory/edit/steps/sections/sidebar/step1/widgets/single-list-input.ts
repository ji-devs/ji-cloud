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
              height: 40px;
            width: 460px; /*arbitrary amount to not go into rounded corners*/
          outline: none;
          border: none;
          font-size: 24px;
          text-align: center; 
        }

        :host([placeholder]) input {
            color: var(--light-gray-4); 
        }

    `];
  }


  onInput(evt:InputEvent) {
      const {value} = (evt.target as any);

      //TODO - limit by grapheme cluster count
    this.value = value;

    this.dispatchEvent(new CustomEvent("custom-input", {
      detail: { value },
    }))
  }
  onChange(evt:InputEvent) {
    const {value} = (evt.target as any);
      //TODO - limit by grapheme cluster count
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

      console.log(value);

      return html`<div class="row">
          <input type="text" .value=${value} @input="${this.onInput}" @change="${this.onChange}" >
          </input>
      </div>`
  }
}
