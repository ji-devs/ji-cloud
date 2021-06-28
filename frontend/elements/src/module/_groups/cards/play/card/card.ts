import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {ThemeKind} from "@elements/_themes/themes";
import { styleMap } from 'lit-html/directives/style-map';
import {cardBackPath, Mode, Side, StyleKind, getContentStyle} from "@elements/module/_groups/cards/helpers";
import {Size, cardStyles} from "./styles";


@customElement('play-card')
export class _ extends LitElement {
  static get styles() {
      return [...cardStyles, css`
      
        :host([hasTransform]) {
            position: absolute;
            top: 0;
            left: 0;
            z-index: 1000;
            transition-duration: 0s;
        }

        :host([side="right"]) > section {
            z-index: 1000;
        }

        :host([side="left"]) > section {
            z-index: 1001;
        }

        section {
            transition: transform 0.8s;
            transform-style: preserve-3d;
            transform: rotateY(180deg);
        }

        .front, .back {
            position: absolute;
            -webkit-backface-visibility: hidden; /* Safari */
            backface-visibility: hidden;
        }

        .back {
            transform: rotateY(180deg);
            justify-content: center;
            align-items: center;
            display: flex;
            width: 100%;
            height: 100%;
        }

        .back > img-ui {
            object-fit: cover;
        }

        .back > img-ui {
            box-sizing: border-box;
            width: 100%;
            height: 100%;
        }

        ::slotted(img-ji) {
            --img-size: calc(
            var(--card-size) - ((var(--border-size) * 2) + var(--img-padding))
            );
            width: var(--img-size);
            height: var(--img-size);
        }

        ::slotted(img-ji), ::slotted(img-ui) {
            object-fit: contain;
        }

        :host([flipped]) > section {
            transform: rotateY(0deg);
        }


      `];
  }

  // whether or not showing front vs. back
  @property({type: Boolean, reflect: true})
  flipped:boolean = false;

  // required for styling
  @property()
  theme:ThemeKind = "blank";

  // required for styling
  @property()
  mode:Mode = "duplicate";

  // required for styling (i.e. "primary" vs. "secondary" in lettering mode)
  @property({reflect: true})
  side:Side = "left";

  // predefined card sizes
  @property({reflect: true})
  size:Size = "memory";

  // if applying a transform, convenient to set all the positioning/z-indexing etc.
  @property({type: Boolean, reflect: true})
  hasTransform:boolean = false;

  // automatically flip it while hovering
  @property({type: Boolean})
  flipOnHover:boolean = false;

  // double-sided cards need the border etc. on the back
  @property({type: Boolean})
  doubleSided:boolean = false;

  // style mode - see helpers definition
  @property()
  styleKind:StyleKind = "theme"

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
      //const {theme, mode, scale, side, doubleSided, transform, translateX, translateY} = this;
      const {theme, mode, side, doubleSided, styleKind} = this;


      const backSide = side === "left" ? "right" : "left";

      //const style = transform ? `transform: scale(${scale}) translate(${translateX}rem, ${translateY}rem);` : nothing;

      return html`
          <section>
              <div class="front content" style=${getContentStyle(styleKind, theme, mode, side)}><slot></slot></div>
              <div class="back">
                ${doubleSided 
                    ? html`<div class="content" style=${getContentStyle(styleKind, theme, mode, backSide)}><slot name="backSideContent"></slot></div>`
                    : html`<img-ui path="${cardBackPath(theme)}"></img-ui>`
                }
                </div>
          </section>
      `
  }
}
