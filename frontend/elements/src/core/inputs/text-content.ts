import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

export type Mode = "passwordVisible" | "passwordHidden" | "text";

@customElement("input-text-content")
export class _ extends LitElement {
  static get styles() {
    return [css``];
  }

  @property()
  value: string = "";

  @property({ type: Boolean })
  editing: boolean = false;

  onKey(evt: KeyboardEvent) {
    let { key } = evt;
    key = key.toLowerCase();
    if (key === "escape") {
      this.editing = false;
    } else if(key === "enter") {
        this.dispatchChange();
    }
  }

  onGlobalMouseDown = (evt: MouseEvent) => {
    if(!evt.composedPath().includes(this as any)) {
        console.log("dispatching?");
        this.dispatchChange();
    }
  }

  updated(changed:any) {
        if(typeof changed.get("editing") === "boolean") {
            const {editing} = this;
            this.removeGlobalListener(); 
            if(editing) {
                window.addEventListener("mousedown", this.onGlobalMouseDown);
                this.shadowRoot?.getElementById("input")?.focus();
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

  dispatchChange = () => {
      const input = this.shadowRoot?.getElementById("input") as HTMLInputElement;
      if(input == null) {
          console.warn("input should never be null!");
      } else {
          const {value} = input;
        this.dispatchEvent(
            new CustomEvent("custom-change", {
                detail: { value},
            })
        );
      }

      this.editing = false;
  }

  render() {
    const { value, editing } = this;

    return editing
      ? html`<input id="input" type="text" @keyup="${this.onKey}" value="${value}"></input>`
      : html`<span @dblclick=${() => this.editing = true}>${value}</span>`;
  }
}