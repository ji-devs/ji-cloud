import { LitElement, svg, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

type TraceEditKind = "path" | "rect" | "circle";

@customElement('trace-edit-reshape-menu')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              position: absolute;
              top: 0;
              left: 0;
          }
    `]
  }

  render() {
      return html`
          <section>
              <slot></slot>
          </section>
      `;
  }
}
