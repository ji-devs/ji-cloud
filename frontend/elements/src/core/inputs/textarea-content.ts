/* This is effectively like a contenteditable
 * but it uses a real textarea element
 * and thereby avoids many of the quirks
 *
 * the implementation uses local refs, so don't rely on the `input` property
 * e.g. only use `input` for setting the _initial_ value
 *
 * Starting in edit mode might be off if waiting for fonts to load
 */

import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

export type CLICK_MODE = "single" | "double" | "none";

@customElement("input-textarea-content")
export class _ extends LitElement {
  static get styles() {
      return [css`

          textarea, span {
              font-family: Poppins;
              font-size: 16px;
              display: none;
              text-align: center;
          }

          span {
              white-space: pre-wrap;
          }
          textarea {
              outline: 0;
              border: none;
              padding: 0;
              resize: none;
              overflow: hidden;
          }

          textarea.visible, span.measure, span.visible {
              display: inline-block;
          }

          span.measure {
              position: absolute;
              left: -10000px;
              bottom: 10000px;
          }

      `];
  }

  @property()
  value: string = "";

  @property({ type: Boolean })
  editing: boolean = false;

  @property()
  clickMode: CLICK_MODE = "double";

  onKey(evt: KeyboardEvent) {
    let { key } = evt;
    key = key.toLowerCase();
    if (key === "escape") {
        const input = this.shadowRoot?.getElementById("input") as HTMLInputElement;
        input.value = this.value;
        this.editing = false;
        this.dispatchEvent(new Event("reset"));
    } else if(key === "enter") {
        //not for textarea...
        //this.dispatchChange();
    }
  }

  onInput() {
      const input = this.shadowRoot?.getElementById("input") as HTMLInputElement;
        this.dispatchEvent(
            new CustomEvent("custom-input", {
                detail: { value: input.value},
            })
        );
      this.resizeInput();
  }

  resizeInput = () => {
      const input = this.shadowRoot?.getElementById("input") as HTMLInputElement;
      const measure = this.shadowRoot?.getElementById("measure") as HTMLInputElement;


      measure.textContent = this.getValue() as string;

      const lastChar = input.value.charAt(input.value.length-1);
      const rect = measure.getBoundingClientRect();

      let {width, height} = rect;
      if(lastChar === '\n' || lastChar === '\r') {
            const measureLine = this.shadowRoot?.getElementById("measure-line") as HTMLInputElement;
            const lineRect = measureLine.getBoundingClientRect();
            height += lineRect.height;
      }

      input.style.width = `${width}px`;
      input.style.height= `${height}px`;
  }

  onGlobalMouseDown = (evt: MouseEvent) => {
    if(!evt.composedPath().includes(this as any)) {
        this.dispatchChange();
    }
  }

  firstUpdated(_changed:any) {
    this.resizeInput();
  }
  updated(changed:any) {
        if(typeof changed.get("editing") === "boolean") {
            const {editing} = this;
            this.removeGlobalListener(); 
            if(editing) {
                window.addEventListener("mousedown", this.onGlobalMouseDown);
                const input = this.shadowRoot?.getElementById("input") as HTMLInputElement;
                if(input) {
                    input.focus();
                    input.value = this.value;
                    input.setSelectionRange(-1, -1);
                    this.resizeInput();
                }
            }
        }
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    this.removeGlobalListener(); 
  }

  removeGlobalListener() {
     window.removeEventListener("mousedown", this.onGlobalMouseDown);
  }

  getValue = () => {
      const input = this.shadowRoot?.getElementById("input") as HTMLInputElement;
      if(input == null) {
          console.warn("input should never be null!");
          return undefined;
      } else {
          const {value} = input;
          return value;
      }

      
  }
  dispatchChange = () => {
      const value = this.getValue();
        this.dispatchEvent(
            new CustomEvent("custom-change", {
                detail: { value},
            })
        );

      this.editing = false;
  }

  render() {
    const { value, editing, clickMode} = this;

    return html`
        <textarea class="${classMap({visible: editing})}" id="input" @input="${this.onInput}" @keyup="${this.onKey}">${value}</textarea>
        <span id="show" class="${classMap({visible: !editing})}"
              @dblclick=${() => {
                if(clickMode === "double") {
                    this.editing = true
                }
              }}
              @click=${() => {
                if(clickMode === "single") {
                    this.editing = true
                }
              }}
              >${value}</span>
        <span id="measure" class="measure">${value}</span>
        <span id="measure-line" class="measure">&nbsp;</span>
        `;
  }
}
