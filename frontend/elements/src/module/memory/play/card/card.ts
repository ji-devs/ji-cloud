import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {ThemeKind} from "@elements/module/_common/theme";
import {cardBackPath} from "@elements/module/memory/_common/helpers";

@customElement('play-card')
export class _ extends LitElement {
  static get styles() {
      return [css`

          :host([transform]) > section {
              position: absolute;
              top: 0;
              left: 0;
          }

          :host([transform]) {
                  cursor: default;
          }
          :host {
                  cursor: pointer;
          }
          section {
              transition: transform 0.8s;
              transform-style: preserve-3d;
              transform: rotateY(180deg);
          }

          section, ::slotted(img-ji), .back > img-ui {
              width: calc(168rem * (1920/1719));
              height: calc(168rem * (1920/1719));
          }

          ::slotted(img-ji), ::slotted(img-ui) {
                object-fit: contain;
            }

          :host([flipped]) > section {
              transform: rotateY(0deg);
          }

          .front, .back {
              justify-content: center;
              align-items: center;
              display: flex;
              position: absolute;
              width: 100%;
              height: 100%;
              -webkit-backface-visibility: hidden; /* Safari */
                  backface-visibility: hidden;
          }

          .front {
              border-radius: 16px;
              border-style: solid;
              border-width: 3px;

              background-color: white;
          }

          .back {
              transform: rotateY(180deg);
          }
            .back > img-ui {
                object-fit: cover;
            }
    `];
  }

  @property({type: Boolean, reflect: true})
  flipped:boolean = false;

  @property()
  theme:ThemeKind= "";

  @property({type: Boolean, reflect: true})
  transform:boolean = false;

  @property({type: Number})
  scale:number = 1;

  @property({type: Number})
  translateX:number = 0;

  @property({type: Number})
  translateY:number = 0;

  render() {
      const {theme, scale, transform, translateX, translateY} = this;

      const frontStyle = `border-color: var(--theme-${theme}-border-color)`;
      const style = transform ? `transform: scale(${scale}) translate(${translateX}rem, ${translateY}rem);` : nothing;

      return html`
          <section style="${style}">
          <div class="front" style="${frontStyle}"><slot></slot></div>
          <div class="back"><img-ui path="${cardBackPath(theme)}"></img-ui></div>
          </section>
      `
  }
}
