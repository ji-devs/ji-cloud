
import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";


@customElement('main-card-pair')
export class _ extends LitElement {
  static get styles() {
      return [css`
        section, .index{
          width: 350px; 
        }

        section {
          border-radius: 24rem;
          height: 236px;
        }

        section.hover {
          background-color: #deecff;
        }

        section.hover > .close {
            display: block;
        }

        .close {
            display: none;
            position: relative;
            top: 0;
            left: 0;
        }
        slot[name="close"]::slotted(*) {
            position: absolute;
            top: -16px;
            left: 333px;
            display: inline-block;
            width: 32px;
            height: 32px;
        }
        
        .cards {
              position: relative;
              top: 24px;
              left: 24px;
          }
          .left {
              position: absolute;
              top: 0;
              left: 0;
          }

          .right {
              position: absolute;
              top: 16px;
              left: 136px;
          }
          .index {
                font-size: 14px;
              text-align: center;
          }
    `];
  }

  onEnter() {
      if(this.hoverable) {
          this.hover = true;
      }
  }

  onLeave() {
      if(this.hoverable) {
          this.hover = false;
      }
  }

  @property({type: Boolean})
  hover:boolean = false;

  @property({type: Number})
  index:number = 0;

  @property({type: Boolean})
  hoverable:boolean = false;

  render() {
      const {hover, index} = this;


      return html`
        <section class="${classMap({hover})}" @mouseenter="${this.onEnter}" @mouseleave="${this.onLeave}">
              <div class="close"><slot name="close"></slot></div>
              <div class="cards">
                  <div class="right"><slot name="right"></slot></div>
                  <div class="left"><slot name="left"></slot></div>
              </div>
        </section>
        <div class="index">${index + 1}</div>
        <slot name="error"></slot>
      `
  }
}
