import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";

@customElement('ellipses-menu-line')
export class _ extends LitElement {
  static get styles() {
    return [css`
      main {
        display: flex; 
        align-items: center;
      }

      .button {
        margin-left: 10px;
        visibility: hidden;
      }

      .button.visible {
        visibility: visible;
      }
      .menu-container {
        display: none;
        position: relative;
      }
      .menu-container.visible {
        display: block;
      }
      .menu {
        position: absolute;
        top: 0px;
        left: 0px;
        background-color: white;

        min-width: 112px;
        padding: 16px;
        border-radius: 4px;
        -webkit-backdrop-filter: blur(30px);
        backdrop-filter: blur(30px);
        box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.25);
        background-color: #ffffff;
      }

    `];
  }

  @property({type:Boolean})
  visible:boolean = false;

  @property({type:Boolean})
  hover :boolean = false;

  onEnter() {
    this.hover = true;
  }
  onLeave() {
    this.hover = false;
  }
  render() {
    const {visible, hover} = this;

      const menuContainerClasses = classMap({
        ["menu-container"]: true,
        visible
      });

      const buttonClasses = classMap({
        button: true,
        visible: hover
      });

      return html`
        <main @mouseenter="${this.onEnter.bind(this)}" @mouseleave="${this.onLeave.bind(this)}">
          <slot name="content"></slot>
          <button-ellipses class="${buttonClasses}" @click=${() => this.visible = !this.visible}></button-ellipses>
          <div class="${menuContainerClasses}">
            <div class="menu">
              <slot name="menu-content"></slot>
            </div>
          </div>
        </main>
      `
        
  }
}