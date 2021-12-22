import { LitElement, html, css, customElement, state } from "lit-element";
import jigFlex from './jig-flex-css';
import "@elements/core/overlays/dialog-overlay";
import "@elements/entry/admin/jig_label_ui/jig-details";

@customElement("single-jig")
export class SingleJig extends LitElement {
  static styles = [
    jigFlex,
    css`
      .jig {
        display: flex;
        justify-content: space-between;
      }

      #overlay-container {
        width: 100%;
      }
    `,
  ];

  @state() open = false;
  
  openOverlay(event: Event) {
    event.preventDefault();
    this.open = true;
  }

  closeOverlay() {
    this.open = false;
  }
  
  render() {
    return html`
    <div class="jig">
      <div class="flex"><a href="" @click=${this.openOverlay}><slot name="jig-name"></slot></a></div>
      <div class="flex"><slot name="author"></slot></div>
      <div class="flex"><slot name="author-badge"></slot></div>
      <div class="flex"><slot name="date"></slot></div>
      <div class="flex"><slot name="language"></slot></div>
      <div class="flex"><slot name="curators"></slot></div>
      <div class="flex"><slot name="age-ranges"></slot></div>
    </div>
    <div id="overlay-container">
      <dialog-overlay @close=${this.closeOverlay} autoClose ?open=${this.open}>
        <jig-details></jig-details>
      </dialog-overlay>
    </div>
    `;
  }
}
