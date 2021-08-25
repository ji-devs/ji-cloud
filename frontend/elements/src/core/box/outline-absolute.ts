import { LitElement, svg, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";


@customElement('box-outline-absolute')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              position: absolute;
              box-shadow: 1px 1px 1px 0 rgba(0, 0, 0, 0.16);
              border: solid 1px var(--main-blue);
          }

          :host([click-area]) ::slotted([slot=click-area]) {
	    display: block;
	    cursor: pointer;
            position: absolute;
            top: 0; 
            left: 0;
            width: 100%;
            height: 100%;
	  }

	  ::slotted([slot=click-area]) {
	    display: none;
	  }


	  ::slotted([slot=top-right]) {
	      display: block;
              position: absolute;
	      /* the rust component takes a mixin to override these */
              top: -16px;
              right: -16px;
	  }

	  :host([top-right-hover-only]:not([hover])) ::slotted([slot=top-right]) {
	      display: none;
	  }
    `]
  }

  @property({type: Boolean, reflect: true})
  "top-right-hover-only":boolean = false;

  @property({type: Boolean, reflect: true})
  "click-area":boolean = false;

    @property({type: Boolean, reflect: true})
    hover:boolean = false;

    connectedCallback() {
      super.connectedCallback();
      this.addEventListener("mouseenter", this.onMouseEnter);
      this.addEventListener("mouseleave", this.onMouseLeave);
    }

    disconnectedCallback() {
      super.disconnectedCallback();
      this.removeEventListener("mouseenter", this.onMouseEnter);
      this.removeEventListener("mouseleave", this.onMouseLeave);
    }

    onMouseEnter() {
      this.hover = true;
    }

    onMouseLeave() {
      this.hover = false;
    }

  render() {
      return html`
          <slot name="click-area"></slot>
	  <slot name="top-right"></slot>
      `;
  }
}
