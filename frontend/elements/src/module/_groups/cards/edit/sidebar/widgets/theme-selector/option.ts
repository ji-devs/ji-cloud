import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {ThemeId, THEMES} from "@elements/_themes/themes";
import {cardBackIconPath} from "@elements/module/_groups/cards/helpers";
import "@elements/module/_common/edit/widgets/theme-selector/jig";

export type STATE = "idle" | "selected" | "jig";

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
          .left, .right {
            box-sizing: border-box;
            width: 112px;
            height: 112px;
          }
          .left {
            position: absolute;
            top: 16px;
            left: 13px;
            border-radius: 16px;
            box-shadow: 0px 3px 6px rgba(0, 0, 0, 0.16);
          }

          .right {
            position: absolute;
            top: 32px;
            left: calc(88px + 13px);
          }

          .right {
            display: flex;
            justify-content: center;
            align-items: center;
          }

          .menu {
            position: absolute;
            top: -16px;
            right: -16px;
            z-index: 1;
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

          .hidden {
            display: none;
          }

          :host([state="selected"]) .label, :host([state="jig"]) .label {
            color: var(--main-blue);
          }
         
          img-ui {
            width: 100%;
            height: 100%; 
            object-fit: cover;
          }
        `,
      ];
  }

  @property()
  theme:ThemeId = "blank";

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

      const sectionClasses = classMap({
        hover: hover && state !== "jig"
      });
      

      const imageClass = classMap({
        hidden: hover
      })
      const imageHoverClass = classMap({
        hidden: !hover
      })
      return html`
        <section class=${sectionClasses} @mouseenter="${this.onEnter.bind(this)}" @mouseleave="${this.onLeave.bind(this)}">
          ${state == "jig" ? html`<theme-selector-jig class="jig"></theme-selector-jig>` : nothing}
              <div class="content">
                  <div class="right">
                    <img-ui path="${cardBackIconPath(theme)}"></img-ui>
                    ${state === "selected" ? renderMenu() : nothing}
                  </div>
                  <div class="left">
                    <img-ui class=${imageClass} path="theme/${theme}/card-front-icon.svg"></img-ui>
                    <img-ui class=${imageHoverClass} path="theme/${theme}/card-front-icon-hover.svg"></img-ui>
                  </div>
                  <div class="label">${THEMES[theme].label.en}</div>
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