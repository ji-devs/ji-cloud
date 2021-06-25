import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

export type Mode = "single" | "double";

const STR_MODE_LABEL:Record<Mode, string> = {
    "single": "Single card",
    "double": "A pair",
}

@customElement('flashcards-settings-option')
export class _ extends LitElement {
  static get styles() {
      return [
        css`

          :host {
            cursor: pointer;
          }
          section {
            width: 238px;
            height: 172px;
            border-radius: 16px;
            border: solid 3px rgba(0, 0, 0, 0);
            box-sizing: border-box;
          }

          section.hover {
            border: solid 3px var(--light-blue-4);
          }


          :host([selected]) section {
            border: solid 3px var(--light-blue-4);
            background-color: var(--light-blue-3);
          }

          .content {
            margin-top: 18px;
            margin-bottom: 8px;
            margin-left: auto;
            margin-right: auto;
            display: flex;
            align-items: center;
            justify-content: center;
            width: 207px;
            height: 118px;
            border-radius: 4px;
            background-color: var(--white);
          }

          .label {
            text-align: center;
            width: 100%;
            font-size: 14px;
            font-weight: 500;
            color: var(--dark-blue-8);
          }

          :host([selected]) .label {
            color: var(--main-blue);
          }
        `,
      ];
  }

  @property()
  mode:Mode = "single";

  @property({type: Boolean, reflect: true})
  selected:boolean = false; 

  @property({type: Boolean, reflect: true})
  hover:boolean = false;

  onEnter() {
    this.hover = true;
  }

  onLeave() {
    this.hover = false;
  }

  render() {
      const {mode, selected, hover} = this;

      const sectionClasses = classMap({
        hover
      });

      return html`
        <section class=${sectionClasses} @mouseenter="${this.onEnter.bind(this)}" @mouseleave="${this.onLeave.bind(this)}">
            <div class="content">
                <img-ui path="module/flashcards/edit/sidebar/${mode}.svg"></img-ui>
            </div>
            <div class="label">${STR_MODE_LABEL[mode]}</div>
          </section>
      `
  }
}