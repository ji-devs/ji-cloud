import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import "@elements/core/images/ui";

type IconKind = ""
| "copy"
| "delete"
| "duplicate"
| "edit"
| "move-down"
| "move-up"
| "paste"
| "print"
| "reuse";

const STR_LABEL_LOOKUP:any = {
    copy: "Copy",
    delete: "Delete",
    duplicate: "Duplicate",
    edit: "Edit",
    ["move-down"]: "Move Down",
    ["move-up"]: "Move Up",
    ["paste"]: "Paste",
    ["print"]: "Print",
    ["reuse"]: "Reuse",
};

@customElement('menu-line')
export class _ extends LitElement {
  static get styles() {
      return [css`
          section {
              display: flex;
              cursor: pointer;
          }

          .img {
              width: 24px;
              height: 24px;
              margin-right: 13px;
              display: flex;
              justify-content: center;
              align-items: center;
          }

          .red {
            color: #f84f57;
          }

    `];
  }

  @property()
  icon: IconKind = "";

  @property()
  customLabel: string = "";

  render() {
    const {icon, customLabel} = this;

    const label = customLabel !== "" ? customLabel
        : STR_LABEL_LOOKUP[icon]; 

    const labelClasses = classMap({label, red: icon === "delete"});

    return html`
        <section>
            ${icon === "" ? nothing
                : html`<div class="img"><img-ui path="core/menus/${icon}.svg" /></div>`
            }
            <div class="${labelClasses}">${label}</div>
        </section>
      `
        
  }
}
