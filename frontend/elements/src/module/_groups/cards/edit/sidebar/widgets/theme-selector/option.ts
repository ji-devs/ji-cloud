import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {ThemeKind, STR_THEME_LABEL} from "@elements/_themes/themes";
import {cardBackPath} from "@elements/module/_groups/cards/helpers";
import "@elements/module/_common/edit/widgets/theme-selector/jig";

export type STATE = "idle" | "selected" | "jig";

const STR_SAMPLE_HEBREW = "אבא";
const STR_SAMPLE_ENGLISH = "Mom";

@customElement('theme-selector-cards-option')
export class _ extends LitElement {
  static get styles() {
      return [
        css`
          :host {
            cursor: pointer;
          }
          section {
            position: relative;
            width: 232px;
            height: 196px;
            border-radius: 16px;
            border: solid 3px rgba(0, 0, 0, 0);
            box-sizing: border-box;
          }

          section.hover {
            border: solid 3px var(--light-blue-4);
          }


          :host([state="selected"]) section {
            border: solid 3px var(--light-blue-4);
            background-color: var(--light-blue-3);
          }

          .content {
            position: relative;
            top: 0px;
            left: 0px;
          }
          .left {
            position: absolute;
            top: 16px;
            left: 13px;
          }

          .right {
            position: absolute;
            top: 32px;
            left: calc(88px + 13px);
          }
          .left,
          .right {
            display: flex;
            justify-content: center;
            align-items: center;
            box-sizing: border-box;
            width: 112px;
            height: 112px;
            box-shadow: 0 3px 20px 0 rgba(0, 0, 0, 0.06);
          }

          .menu {
            position: absolute;
            top: -16px;
            right: -16px;
            z-index: 1;
          }

          .left {
            border: solid 3px #fa632f;
            border-radius: 16px;
            background-color: var(--white);
          }

          .label {
            position: absolute;
            top: 160px;
            left: 0px;
            text-align: center;
            width: 229px;
            font-size: 14px;
            font-weight: 500;
            color: var(--dark-blue-8);
          }

          :host([state="selected"]) .label, :host([state="jig"]) .label {
            color: var(--main-blue);
          }
          
          img-ui {
            width: 109px;
            height: 109px;
            object-fit: cover;
          }
        `,
      ];
  }

  @property()
  theme:ThemeKind = "blank";

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

      const style = `border-color: var(--theme-${theme}-color-2)`;

      const text = hover ? STR_SAMPLE_HEBREW : STR_SAMPLE_ENGLISH;

      const sectionClasses = classMap({
        hover: hover && state !== "jig"
      });
      

      return html`
        <section class=${sectionClasses} @mouseenter="${this.onEnter.bind(this)}" @mouseleave="${this.onLeave.bind(this)}">
          ${state == "jig" ? html`<theme-selector-jig class="jig"></theme-selector-jig>` : nothing}
              <div class="content">
                  <div class="right" style="${style}">
                    <img-ui path="${cardBackPath(theme)}"></img-ui>
                    ${state === "selected" ? renderMenu() : nothing}
                  </div>
                  <div class="left" style="${style}"><span>${text}</span></div>
                  <div class="label">${STR_THEME_LABEL[theme]}</div>
              </div>
          </section>
      `
  }
}

function renderMenu() {
  return html`
        <menu-kebab class="menu" offsetVertical="0" offsetHorizontal="32">
          <slot name="menu"></slot>
        </menu-kebab>
  `
}