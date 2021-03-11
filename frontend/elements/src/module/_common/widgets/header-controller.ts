import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

const STR_PREVIEW = "Preview";

@customElement('module-header-controller')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              display: flex;
              padding: 8px 20px 8px 16px;
              border-radius: 24px;
              box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
              background-color: var(--white);
          }
          section {
              display: flex;
              align-items: center;
              justify-content: center;
          }

          .arrows {
              /* Not sure why, but this looks more centered */ 
              margin-top: 3px;
              display: flex;
              gap: 10px;
          }
          img-ui {
              cursor: pointer;
          }

          .preview {
              /* Not sure why, but this looks more centered */ 
              margin-top: 4px;
              display: flex;
              gap: 8px;
          }
          .divider {
              margin: 0 16px;
              height: 32px;
              border: solid 1px #606060;
          }
    `];
  }

  render() {

      return html`
          <section>
              <div class="arrows">
                  <img-ui path="module/_common/header/undo.svg"></img-ui>
                  <img-ui path="module/_common/header/redo.svg"></img-ui>
              </div>
              <div class="divider"></div>
              <div class="preview">
                  <img-ui path="module/_common/header/play.svg"></img-ui>
                  <div class="preview-label">${STR_PREVIEW}</div>
              </div>
          </section>
      `
  }
}
/*
 */
