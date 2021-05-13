/*
 * This container is encapsulated in an element
 * because the difference between it being dragged around or not is irrelevant
 *
 * will dispatch a close event when clicked outside
 */

//amount of pixels moved after which we disable the child pointer events
const DISABLE_CHILD_POINTER_THRESHHOLD = 3;

import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from 'lit-html';

@customElement('drag-container')
export class _ extends LitElement {
  static get styles() {
    return [css`
      :host {
        position: absolute;
        top: 0px;
        left: 0px;
        cursor: pointer;
      }

          ::slotted(*) {
            user-drag: none; 
            user-select: none;
            -moz-user-select: none;
            -webkit-user-drag: none;
            -webkit-user-select: none;
            -ms-user-select: none;
          }

              :host([disableChildPointer]) ::slotted(*) {
                pointer-events: none;
              }

      `];

  }

  @property({type: Boolean, reflect: true})
  disableChildPointer:boolean = false;

  @property({type: Number})
  x:number = 0;

  @property({type: Number})
  y:number = 0;


  lastMouseX:number = 0;
  lastMouseY:number = 0;

  accumDiffX:number = 0;
  accumDiffY:number = 0;

  onThisMouseDown = (evt: MouseEvent) => {
      this.accumDiffX = 0;
      this.accumDiffY = 0;
        this.disableChildPointer = false;

      this.lastMouseX = evt.clientX;
      this.lastMouseY = evt.clientY;

      window.addEventListener("mousemove", this.onGlobalMouseMove);
      window.addEventListener("mouseup", this.onGlobalMouseUp);
  }

  onGlobalMouseMove = (evt: MouseEvent) => {
        const diffX = evt.clientX - this.lastMouseX;
        const diffY = evt.clientY - this.lastMouseY;

        this.x += diffX;
        this.y += diffY;

        this.lastMouseX = evt.clientX;
        this.lastMouseY = evt.clientY;

        this.accumDiffX += Math.abs(diffX);
        this.accumDiffY += Math.abs(diffX);

        if(this.accumDiffX > DISABLE_CHILD_POINTER_THRESHHOLD || this.accumDiffY >= DISABLE_CHILD_POINTER_THRESHHOLD) {
            this.disableChildPointer = true;
        }
  }

  onGlobalMouseDown = (evt: MouseEvent) => {
      const self = this.shadowRoot?.getElementById("section") as any;

      if(!evt.composedPath().includes(self)) {
          this.dispatchEvent(new Event("close"));
      }
  }

  onGlobalMouseUp = (_evt: MouseEvent) => {
    this.removeGlobalMoveListener();
    this.removeGlobalUpListener();
    this.disableChildPointer = false;
  }

  removeGlobalDownListener() {
     window.removeEventListener("mousedown", this.onGlobalMouseDown);
  }

  removeGlobalMoveListener() {
     window.removeEventListener("mousemove", this.onGlobalMouseMove);
  }
  removeGlobalUpListener() {
     window.removeEventListener("mouseup", this.onGlobalMouseUp);
  }

  //lifecycle
  firstUpdated() {
      window.addEventListener("mousedown", this.onGlobalMouseDown);
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    this.removeGlobalDownListener(); 
    this.removeGlobalMoveListener();
    this.removeGlobalUpListener();
  }

  render() {
      const {x, y} = this;

      const style = `transform: translate(${x}px, ${y}px);`

    return html`<section id="section" style="${style}" @mousedown=${this.onThisMouseDown}><slot></slot></section>`;
  }
}
