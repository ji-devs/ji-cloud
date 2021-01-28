import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { colorStyles, colorValues } from "@elements/_styles/colors";

export type Color =  "white" | "pink"  ;
export type Size = "small" | "large" ;


@customElement("button-circle")
export class CircleButton extends LitElement {
  static get styles() {
    return [
      colorStyles,
      css`
        main {
          cursor: pointer;
          display: flex;
          flex-direction: column;
          align-items: center;
          margin-left: 8px;
          margin-right: 8px;
          width: 84px;
          color: #4a4a4a;
        }

        .circle {
          border-radius: 9999px;
          height: 48px;
          width: 48px;
          border-style: solid;
          border-width: 1px;
          border-color: ${colorValues.grey};
          justify-content: center;
          align-items: center;
          display: flex;
        }

        .circle > * {
          text-align: center;
          width: 100%;
        }

        .circle.disabled {
          background-color: white;
        }

        .circle.active {
          background-color: ${colorValues.blue};
          border: 0;
        }
        .label-active {
          color: ${colorValues.blue};
        }


        .pink{
          background-color:#fd6b71 ;
          border-width: 0px;

        } 

      .white{
        background-color:#ffffff;
        border-width: 0px;

      }  
        
      .large{
        height: 48px;
        width: 48px;
      }

      .small{
        width: 16px;
        height: 16px;
      }


      `,
    ];
  }

  @property({ type: Boolean })
  active: boolean = false;

  @property({ type: Boolean })
  disabled: boolean = false;

  @property()
  label: string = "";

  @property()
  color: Color = "white";

  @property()
  size: Size = "large";

  // Define the element's template
  render() {
    const { active, disabled, label,color,size } = this;

    const circleClasses = classMap({
      circle: true,
      active,
      disabled,
      inactive: !active,
      [size]: true,
      [color]: true,
    });

    const textClasses = classMap({
      ["text-white"]: active,
    });

    const labelClasses = classMap({
      ["label-active"]: active,
    });

    return html`
      <main>
        <div class="${circleClasses}">
          <p class="${textClasses}"><slot></slot></p>
        </div>
        <p class="${labelClasses}">${label}</p>
      </main>
    `;
  }
}
