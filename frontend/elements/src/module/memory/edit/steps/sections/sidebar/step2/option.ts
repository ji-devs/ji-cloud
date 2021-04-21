import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {MODE} from "@elements/module/memory/_common/types";
import {ThemeKind} from "@elements/_themes/themes";
import {cardBackPath} from "@elements/module/memory/_common/helpers";

export type STATE = "idle" | "hover" | "selected";

const STR_SAMPLE_HEBREW = "אבא";
const STR_SAMPLE_ENGLISH = "Mom";

@customElement('step2-sidebar-option')
export class _ extends LitElement {
  static get styles() {
      return [css`
        section {
          width: 235px; 
          height: 196px;
          border-radius: 16px;
            border: solid 3px rgba(0,0,0,0); 
            box-sizing: border-box;
        }

        :host([state="hover"]) section {
            border: solid 3px var(--light-blue-4);
        }

        :host([state="selected"]) section {
            border: solid 3px var(--light-blue-4);
            background-color: var(--light-blue-3);
        }

        .content{
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
          .left, .right {
              display: flex;
              justify-content: center;
              align-items: center;
            box-sizing: border-box;
          width: 112px;
          height: 112px;
          box-shadow: 0 3px 20px 0 rgba(0, 0, 0, 0.06);
          
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
            color: var(--dark-blue-8);
          }

          .selected .label {
            color: var(--main-blue);
          }
        img-ui {
          width: 109px;
          height: 109px;
            object-fit: cover;
        }


    `];
  }

  @property()
  theme:ThemeKind= "";

  @property({reflect: true})
  state:STATE= "idle";

  prevState:STATE | undefined = undefined;

  onEnter() {
    this.prevState = this.state;
    this.state = "hover"; 
  }

  onLeave() {
    this.state = this.prevState as STATE;
  }

  render() {
      const {theme, state} = this;

      const style = `border-color: var(--theme-${theme}-color-2)`;

      const text = state === "hover" ? STR_SAMPLE_HEBREW : STR_SAMPLE_ENGLISH;

      return html`
        <section @mouseenter="${this.onEnter.bind(this)}" @mouseleave="${this.onLeave.bind(this)}">
              <div class="content">
                  <div class="right" style="${style}"><img-ui path="${cardBackPath(theme)}"></img-ui></div>
                  <div class="left" style="${style}"><span>${text}</span></div>
                  <div class="label">${theme}</div>
              </div>
          </section>
      `
  }
}
