import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {ThemeKind} from "@elements/_themes/themes";
import {cardBackPath, Mode, getFrontStyle} from "@elements/module/_groups/cards/helpers";
import { styleMap } from 'lit-html/directives/style-map';

@customElement('main-card')
export class _ extends LitElement {
  static get styles() {
      return [css`

            :host {
                --card-size: 160px;
                --border-size: 1px;
                --img-padding: 10px;
            }
          section {
              transition: transform 0.8s;
              transform-style: preserve-3d;
          }

          :host([dragOver]) section.editing .front {
              border-style: dashed;
              border-radius: 16px;
              border-width: 3px;
              background-color: var(--light-blue-1);
          }

          .front {
              border-style: solid; 
              border-radius: 16px;
              border-width: var(--border-size); 
              background-color: white;
          }
          
          .front, .back, .back > img-ui {
              box-sizing: border-box;
              width: 100%; 
              height: 100%;
          }

          section {
              width: var(--card-size);
              height: var(--card-size);
          }

          ::slotted(img-ui) {
              width: 56px;
              height: 56px;
          }

          ::slotted(img-ji) {
              --img-size: calc(var(--card-size) - ((var(--border-size) * 2) + var(--img-padding)));
              width: var(--img-size); 
              height: var(--img-size); 
              object-fit: contain;

          }

          :host([inverted]) section {
              transform: rotateY(180deg);
          }

          :host([inverted]) section.flippable:hover {
              transform: rotateY(0);
          }
          section.flippable:hover {
              transform: rotateY(180deg);
          }

          .front, .back {
              justify-content: center;
              align-items: center;
              display: flex;
              position: absolute;
              -webkit-backface-visibility: hidden; /* Safari */
                  backface-visibility: hidden;
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
  dragOver:boolean = false;

  @property({type: Boolean})
  flippable:boolean = false;

  @property({type: Boolean, reflect: true})
  inverted:boolean = false;

  @property()
  theme:ThemeKind = "blank";

  @property({type: Boolean})
  editing: boolean = false;

  @property()
  mode:Mode = "duplicate";

  render() {
      const {flippable, theme, editing, mode} = this;

      const frontStyle = getFrontStyle(theme, mode);

      return html`
          <section class="${classMap({flippable, editing})}" >
          <div class="front" style=${frontStyle}><slot></slot></div>
          <div class="back"><img-ui path="${cardBackPath(theme)}"></img-ui></div>
          </section>
      `
  }
}
