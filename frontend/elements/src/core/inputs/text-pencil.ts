import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

@customElement("input-text-pencil")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
      .wrapper {
        position: relative;
        border-radius: 8px;
        border: solid 1px var(--Light_Blue_5);
      }
        input {
          width: calc(100% - 40px);
          margin: 7px 8px 8px 7px;
          outline: none;
          border: none;
          box-shadow : none;  

          font-family: Poppins;
            font-size: 16px;
            font-weight: normal;
            font-stretch: normal;
            font-style: normal;
            line-height: 1.56;
            letter-spacing: normal;
            text-align: left;
            color: var(--Dark_Gray_6);
        }

        .wrapper.editing {
            border: solid 2px var(--Dark_Blue_3);
        }

        .icon {
          position: absolute;
          right: 5px;
          top: 2px;
        }

        .icon.editing {
        }
      `,
    ];
  }

  @property()
  value: string = "";

  @property({type: Boolean})
  editing: boolean = false;

  @property()
  placeholder: string = "";

  onInput(evt:InputEvent) {
    const {value} = (evt.target as any);
    this.value = value;

    this.dispatchEvent(new CustomEvent("custom-input", {
      detail: { value },
    }))
  }

  onFocus(_evt:any) {
      this.editing = true;
  }

  onBlur(_evt:any) {
      this.editing = false;
  }

  render() {
    const { placeholder, value, editing } = this;

    const icon = `core/inputs/pencil-blue-${editing ? "darker" : "lighter"}.svg`;

    const wrapperClasses= classMap({wrapper: true, editing});
    const inputClasses = classMap({editing});
    const iconClasses = classMap({icon: true, editing});

    return html`
        <div class="${wrapperClasses}">
        <input
          placeholder="${placeholder}"
          type="text"
          class="${inputClasses}"
          value="${value}"
          @input="${this.onInput}"

          @focus ="${this.onFocus}"
          @blur ="${this.onBlur}"
        />
        <img-ui class="${iconClasses}" path="${icon}"></img-ui>
        </div>
    `;
  }
}
