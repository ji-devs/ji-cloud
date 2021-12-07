import { LitElement, html, css, customElement } from "lit-element";
import jigFlex from './jig-flex-css';

@customElement("single-jig")
export class SingleJig extends LitElement {
  static styles = [
    jigFlex,
    css`
      .jig {
        display: flex;
        justify-content: space-between;
      }
    `,
  ];

  render() {
    return html`<div class="jig">
      <div class="flex"><slot name="jig-name"></slot></div>
      <div class="flex"><slot name="author"></slot></div>
      <div class="flex"><slot name="author-badge"></slot></div>
      <div class="flex"><slot name="date"></slot></div>
      <div class="flex"><slot name="language"></slot></div>
      <div class="flex"><slot name="curators"></slot></div>
    </div>`;
  }
}
