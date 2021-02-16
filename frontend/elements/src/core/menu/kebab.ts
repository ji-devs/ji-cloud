import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";

import "@elements/core/buttons/icon";

@customElement('menu-kebab')
export class _ extends LitElement {
  static get styles() {
    return [css`
        .menu-container {
            display: none;
                position: fixed;
                top: 0;
                left: 0;
              border-radius: 8px;
              -webkit-backdrop-filter: blur(30px);
              backdrop-filter: blur(30px);
              box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.2);
              background-color: var(--white);
              padding: 14px 16px 16px 14px;
        }
        .menu-container.visible {
            display: block;
        }

        #button {
            width: 32px;
            height: 32px;
        }


    `];
  }

  buttonRef:any;

  @property({type:Boolean})
  visible:boolean = false;


  onGlobalMouseDown = (evt: MouseEvent) => {
    if(!evt.composedPath().includes(this.shadowRoot?.getElementById("menu-container") as any)) {
      this.visible = false;
    }
  }

  firstUpdated(_changed:any) {
      this.buttonRef = this.shadowRoot?.getElementById("button");
      this.requestUpdate();
  }

  updated(changed:any) {
        if(typeof changed.get("visible") === "boolean") {
            const {visible} = this;
            this.removeGlobalListener(); 
            if(visible) {

                window.addEventListener("mousedown", this.onGlobalMouseDown);
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

  getMenuContainerStyle() {
      const {buttonRef, visible} = this;

      if(buttonRef == null) {
          return "display: none;";
      }

      const domRect = buttonRef.getBoundingClientRect(); 

      const {top, right} = domRect;
      return `top: ${top + 8}px; left: ${right + 40}px`;
  }

  render() {
      const {visible} = this;

      const menuContainerClasses = classMap({
        ["menu-container"]: true,
        visible
      });

      const menuButtonIcon = visible ? "circle-kebab-blue" : "circle-kebab-grey";

      return html`
        <section>
            <button-icon id="button" icon="${menuButtonIcon}" @click=${() => this.visible = !this.visible}></button-icon>
              <div id="menu-container" class="${menuContainerClasses}" style="${this.getMenuContainerStyle()}">
                <div class="menu">
                  <slot name="menu-content"></slot>
                </div>
              </div>
        </section>
      `
        
  }
}
