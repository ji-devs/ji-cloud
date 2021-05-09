import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {ThemeKind} from "@elements/_themes/themes";
import {bgIconPath} from "@elements/module/poster/_common/helpers";

export type STATE = "idle" | "selected";

@customElement('step1-sidebar-option')
export class _ extends LitElement {
  static get styles() {
      return [css`
        section {
          width: 232px; 
          height: 196px;
          border-radius: 16px;
            border: solid 3px rgba(0,0,0,0); 
            box-sizing: border-box;
            display: flex;
            justify-contents: center;
            align-items: center;

            flex-direction: column;
        }

        section.hover {
            border: solid 3px var(--light-blue-4);
        }

        :host([state="selected"]) section {
            border: solid 3px var(--light-blue-4);
            background-color: var(--light-blue-3);
        }

        .label {
            text-align: center;
            color: var(--dark-blue-8);
              font-size: 14px;
              font-weight: 500;
              color: var(--dark-blue-8);
        }

        .selected .label {
            color: var(--main-blue);
        }
        img-ui {
            margin-top: 30px;
            margin-bottom: 16px;
            width: 200px;
            height: 114px;
            object-fit: cover;
        }


    `];
  }

  @property()
  theme:ThemeKind= "";

  @property({reflect: true})
  state:STATE= "idle";

  @property({type: Boolean})
  hover:boolean = false;

  onEnter() {
    this.hover = true;
  }

  onLeave() {
    this.hover = false;
  }

  render() {
      const {theme, state, hover} = this;

      return html`
        <section class="${classMap({hover})}" @mouseenter="${this.onEnter.bind(this)}" @mouseleave="${this.onLeave.bind(this)}">
            <img-ui path="${bgIconPath(theme)}"></img-ui>
            <div class="label">${theme}</div>
          </section>
      `
  }
}
