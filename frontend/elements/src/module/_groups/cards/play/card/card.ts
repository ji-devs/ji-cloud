import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {ThemeKind} from "@elements/_themes/themes";
import { styleMap } from 'lit-html/directives/style-map';
import {cardBackPath, Mode, getFrontStyle} from "@elements/module/_groups/cards/helpers";

export type Size = "regular" | "flashcards" | "quiz-option" | "quiz-target";
export type Side = "left" | "right";

@customElement('play-card')
export class _ extends LitElement {
  static get styles() {
      return [css`

            :host {
                --img-padding: 10rem;

            }

            :host([size="flashcards"]) {
                --card-size: 500rem;
                --border-size: 16rem;
            }
            :host([size="regular"]) {
                --card-size: 188rem;
                --border-size: 3rem;
            }
            :host([size="quiz-target"]) {
                --card-size: 431rem;
                --border-size: 4.75rem;
            }
            :host([size="quiz-option"]) {
                --card-size: 253rem;
                --border-size: 4.75rem;
            }
          :host([transform]) > section {
              position: absolute;
              top: 0;
              left: 0;
              z-index: 1000;
              transition-duration: 0s;
          }

          :host([transform]) {
              cursor: default;
          }

          :host([side="right"]) > section {
                  z-index: 1000;
          }
          :host([side="left"]) > section {
                  z-index: 1001;
          }
          :host {
                  cursor: pointer;
          }
          section {
              transition: transform 0.8s;
              transform-style: preserve-3d;
              transform: rotateY(180deg);
              width: var(--card-size);
              height: var(--card-size);
          }

          .front, .back, .back > img-ui {
              box-sizing: border-box;
              width: 100%; 
              height: 100%;
          }
          
          ::slotted(img-ji) {
            --img-size: calc(var(--card-size) - ((var(--border-size) * 2) + var(--img-padding)));
            width: var(--img-size); 
            height: var(--img-size); 
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
              border-width: var(--border-size);
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
  theme:ThemeKind = "blank";

  @property()
  mode:Mode = "duplicate";

  @property({type: Boolean, reflect: true})
  transform:boolean = false;

  @property({type: Number})
  scale:number = 1;

  @property({type: Number})
  translateX:number = 0;

  @property({type: Number})
  translateY:number = 0;

  @property({reflect: true})
  side:Side = "left";

  @property({reflect: true})
  size:Size = "regular";

  @property({type: Boolean})
  flipOnHover:boolean = false;

  connectedCallback() {
      super.connectedCallback();

      this.addEventListener("mouseenter", this.onMouseEnter);
      this.addEventListener("mouseleave", this.onMouseLeave);
  }

  disconnectedCallback() {
      super.disconnectedCallback();

      this.removeEventListener("mouseenter", this.onMouseEnter);
      this.removeEventListener("mouseleave", this.onMouseLeave);
  }

  onMouseEnter() {
      if(this.flipOnHover) {
        this.flipped = !this.flipped;
      }
  }

  onMouseLeave() {
      if(this.flipOnHover) {
        this.flipped = !this.flipped;
      }
  }

  render() {
      const {theme, mode, scale, transform, translateX, translateY} = this;

      const frontStyle = getFrontStyle(theme, mode); 

      const style = transform ? `transform: scale(${scale}) translate(${translateX}rem, ${translateY}rem);` : nothing;

      return html`
          <section style=${style}>
              <div class="front" style=${frontStyle}><slot></slot></div>
              <div class="back"><img-ui path="${cardBackPath(theme)}"></img-ui></div>
          </section>
      `
  }
}
