import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {arrayIndex} from "@utils/array";

@customElement('sidebar-widget-dual-list')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              display: flex;
              gap: 12px;
          }
    `];
  }

  render() {
      return html`
              <slot></slot>
      `
  }
}
