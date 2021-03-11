import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {MODE} from "@elements/module/memory/_common/types.ts";

@customElement('step1-sidebar-empty')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              display: block;
              background-color: red;
              height: 100%;
              position: relative;
          }
          img-ui {

        }

    `];
  }

  @property()
  mode:MODE = "duplicate";

  render() {
      const {mode} = this;

      return html`
          <section>
              <img-ui path="module/memory/sidebar/jiggling-card-pointer.svg"></img-ui>
          </section>
      `
  }
}
