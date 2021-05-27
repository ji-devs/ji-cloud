import { LitElement, svg, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

type TraceEditKind = "path" | "rect" | "circle";

@customElement('trace-edit-select-box')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              position: absolute;
              pointer-events: none;
          }

          :host([selected]) {
              pointer-events: initial;
              box-shadow: 1px 1px 1px 0 rgba(0, 0, 0, 0.16);
              border: solid 1px var(--main-blue);
          }

          .hidden {
              display: none;
          }
          .menu-buttons {
              position: relative;
              top: -40px;
              display: flex;
              justify-content: space-between;
          }

    `]
  }

  @property({type: Boolean, reflect: true})
  selected:boolean = false;

  render() {
        const {selected} = this;

        const menuClasses = classMap({hidden: !selected});

      return html`
          <section class="menu-buttons">
              <slot name="status-btn"><div>&nbsp;</div></slot>
              <div class=${menuClasses}><slot name="menu-btn"></slot></div>
        </section>
      `;
  }
}
