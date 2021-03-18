import { LitElement, html, css, customElement, property} from 'lit-element';

export type SIZE = "small" | "normal";

@customElement('button-flex')
export class FlexButton extends LitElement {
  static get styles() {
      return [css`
          :host {
              display: flex;
              cursor: pointer;
              gap: 8px;
              align-items: center;
          }

        `];
  }

  render() {
      return html`<slot></slot>`;
  }
}
