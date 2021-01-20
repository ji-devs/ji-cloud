import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { BaseButton } from "@elements/_styles/buttons";
import {nothing} from "lit-html";

export type Color = "red" | "blue" | "white" | "green";
export type Size = "small" | "medium" | "large";
export type Weight = "normal" | "medium" | "bold";

@customElement("button-text")
export class _ extends BaseButton {
  static get styles() {
    return [
      css`
        div {
          cursor: pointer;
        }
        .small {
          padding: 0;
        }
        .padding-medium {
          padding: 13.6px 24px 11.4px;
        }
        .padding-large {
          padding: 15px 40px 16px;
        }

        .red {
          color: #fd6b71;
        }

        .red:hover {
          color: #ed6065;
        }

        .blue {
          color: #5590fc;
        }

        .blue:hover {
          color: #387af4;
        }

        button:disabled {
          color: #a9b1b5;
        }

        .weight-bold {
          font-weight: 700;
        }
        .weight-normal {
          font-weight: 400;
        }

        .weight-medium{
          font-weight: 500;
        }
        .green {
          color: #71cf92;
        }
        .green:hover {
          color: #46ba6f;
        }
      `,
    ];
  }

  @property()
  size: Size = "medium";

  @property()
  color: Color = "blue";

  @property()
  weight: Weight = "normal";

  @property({ type: Boolean })
  italic: boolean = false;

  @property({ type: Boolean })
  p: boolean = false;
  render() {
    const { size, color, weight, italic, p } = this;

    const classes = classMap({
      [`weight-${weight}`]: true,
      [size]: true,
      [color]: true,
      italic: italic,
    });

    return html`

      ${p ? html`<p>`: nothing}

      <div class="${classes}">
          <slot></slot>
      </div>

      ${p ? html`</p>` : nothing}
    `;
  }
}
