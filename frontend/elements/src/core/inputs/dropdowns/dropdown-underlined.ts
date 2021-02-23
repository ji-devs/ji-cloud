import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

@customElement("dropdown-underlined")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
      section.selected {
          display: flex;
          justify-content: space-between;
          border-bottom: solid 1px #d3d3d3;
          cursor: pointer;
      }
      .arrow.open {
          transform: rotate(90deg);
      }
      section.options {
          display: none;
          box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
          width: 100%;
          border-radius: 0 0 14px 14px;
          padding-top: 10px;
          cursor: pointer;
      }
      section.options.open {
          display: flex;
          flex-direction: column;
      }
      `,
    ];
  }

 
  @property()
  value: string = "";

  @property()
  placeholder: string = "";

  @property({ type: Boolean })
  open: boolean = false;

  @property({ type: Number })
  maxChildrenHeight: number = 400;

  render() {
    const {value, placeholder, open} = this; 
    return html`
        <section @click=${() => this.open = !this.open} class="selected">
            ${value !== ""
                ? html`<div class="value">${value}</div>`
                : html`<div class="placeholder">${placeholder}</div>`
            }
            <img-ui class="arrow ${open ? "open" : ""}" path="core/_common/chevron-right-grey.svg"></img-ui>
        </section>
            <section class="options ${open ? "open" : ""}">
                <slot name="options"></slot>
            </section>

    `;
  }
}
