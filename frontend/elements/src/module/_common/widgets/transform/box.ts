import { LitElement, svg, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

type CirclePos = 
    "tl" | "t" | "tr"
    | "l" | "r"
    | "bl" | "b" | "br"
    

@customElement('transform-box')
export class _ extends LitElement {
  static get styles() {
      return [css`
          svg {
              position: absolute;
              top: 0;
              left: 0;
              z-index: 1;
          }


          #rotLine {
            stroke: var(--main-blue);
            stroke-width: 1;
          }

          #rotButton {
              cursor: pointer;
          }

          #rect {
              cursor: move;
            fill-opacity: 0;
            stroke: var(--main-blue);
            stroke-width: 3;
            stroke-dasharray: 4px;
          }

          .circle {
              position: absolute;
              top: 0;
              left: 0;
            fill: var(--main-blue);
          }

          .circle.tl {
              cursor: se-resize;
          }
          .circle.tr {
              cursor: ne-resize;
          }
          .circle.bl {
              cursor: sw-resize;
          }
          .circle.br {
              cursor: se-resize;
          }

          .circle.t, circle.b {
              cursor: n-resize;
          }
          .circle.l, circle.r {
              cursor: e-resize;
          }
    `];
  }

  onResizeStart(pos:CirclePos, evt:MouseEvent) {
    this.dispatchEvent(new CustomEvent("transform-resize-start", {
        detail: {
            pos,
            x: evt.clientX,
            y: evt.clientY,
        }
    }));
  }

  onMoveStart(evt:MouseEvent) {
    this.dispatchEvent(new CustomEvent("transform-move-start", {
        detail: {
            x: evt.clientX,
            y: evt.clientY,
        }
    }));
  }

  onRotateStart(evt:MouseEvent) {
    this.dispatchEvent(new CustomEvent("transform-rotate-start", {
        detail: {
            x: evt.clientX,
            y: evt.clientY,
        }
    }));
  }

  @property({type: Number})
  width:number = 0;

  @property({type: Number})
  height:number = 0;

  @property({type: Number})
  radius:number = 10;

  @property({type: Number})
  rotLineDistance:number = 30;

  @property()
  unit:"px" | "rem" = "px";
  
  render() {
      const {width, height, radius, unit, rotLineDistance} = this;

      const withUnit = (value:number) => `${value}${unit}`;


      const renderMain= () => {
          const padWidth = width + (radius * 2);
          const padHeight = height+ (radius * 2);
          const circlePositions:Record<CirclePos, [number, number]> = {
                "tl": [radius, radius],
                "t": [padWidth / 2, radius],
                "tr": [padWidth - radius, radius],
                "l": [radius, padHeight / 2],
                "bl": [radius, padHeight - radius],
                "b": [padWidth / 2, padHeight - radius],
                "br": [padWidth - radius, padHeight - radius],
                "r": [padWidth - radius, padHeight  / 2],
          };
          const renderCircle = (pos:CirclePos) => {
              const [x, y] = circlePositions[pos];
                
              return svg`
              <circle class="circle ${pos}" cx="${withUnit(x)}" cy="${withUnit(y)}" r="${withUnit(radius)}" @mousedown=${(evt:MouseEvent) => this.onResizeStart(pos, evt)} />
              `;
          }

          const circleIds:Array<CirclePos> = ["tl", "t", "tr", "l", "r", "bl", "b", "br"];

          return svg`<svg width="${withUnit(padWidth)}" height="${withUnit(padHeight)}" style="left: ${withUnit(-radius)}; top: ${withUnit(-radius)}">

                <rect id="rect" x="${withUnit(radius)}" y="${withUnit(radius)}" width="${withUnit(width)}" height="${withUnit(height)}" @mousedown=${this.onMoveStart} />
            ${circleIds.map(renderCircle)}
          </svg>`
      }

      const renderRot = () => {
          const renderRotLine= () => {
              const lineWidth = 2;
              const x = width / 2; 
              const y1 = 0; 
              const y2 = -rotLineDistance;

              const lineHeight = rotLineDistance;

              return svg`
                <svg width="${withUnit(lineWidth)}" height="${withUnit(lineHeight)}" style="left: ${withUnit(width/2)}; top: ${withUnit(-(radius + lineHeight))}">
                    <line id="rotLine" x1="0" y1="0" x2="0" y2="${withUnit(lineHeight)}" />
                </svg>
                `;
          }
          const renderRotButton = () => {
              const lineWidth = 2;
              const x = width / 2; 
              const y1 = 0; 
              const y2 = -rotLineDistance;

              const lineHeight = rotLineDistance;

              return svg`
              <svg id="rotButton" width="32" height="32" style="left: ${withUnit((width - 32)/2)}; top: ${withUnit(-(radius + lineHeight + 32))}" @mousedown=${this.onRotateStart}>
                <g id="Ellipse_393" fill="#fff" stroke="#5590fc" transform="translate(1 1)">
                        <circle cx="15" cy="15" r="15" stroke="none"/>
                        <circle cx="15" cy="15" r="14.5" class="cls-1"/>
                    </g>
                    <path id="Path_123370" fill="#5590fc" stroke="#5590fc" stroke-width="0.2px" d="M539.175 676.16a.391.391 0 0 0-.494.239 6.541 6.541 0 0 1-1.119 1.984 6.722 6.722 0 0 1-8.752 1.414 6.52 6.52 0 0 1-1.228-1.04 6.72 6.72 0 0 1-.979-7.677 5.694 5.694 0 0 1 .521-.8c.09-.112.562-.721.778-.927a6.5 6.5 0 0 1 8.934-.01l-3.522.748a.385.385 0 0 0 .082.764.441.441 0 0 0 .077-.009l4.346-.923a.391.391 0 0 0 .231-.146c0-.005 0-.013.008-.018a.384.384 0 0 0 .06-.13v-.011a.392.392 0 0 0 .012-.112l-.332-4.43a.385.385 0 0 0-.132-.264.394.394 0 0 0-.281-.092.388.388 0 0 0-.357.413l.268 3.585a7.256 7.256 0 0 0-9.819-.026l-.1.095a7.491 7.491 0 0 0-1.436 1.9 7.255 7.255 0 0 0 1.67 9.2l.108.087c.073.058.15.105.224.16.059.045.121.085.18.128.079.055.156.114.237.165a7.382 7.382 0 0 0 .68.407.373.373 0 0 0 .12.029 7.5 7.5 0 0 0 9-2 7.32 7.32 0 0 0 1.251-2.215.385.385 0 0 0-.241-.49z" transform="translate(-516.98 -657.662)"/>
                </svg>
                `;
          }

          return html`${renderRotLine()} ${renderRotButton()}`
      }

      return html`${renderMain()} ${renderRot()}`;
  }
  /*
  render() {
      const {width, height, radius, unit, rotLineDistance} = this;

      const fullWidth = width + (radius * 2);
      const fullHeight = height + (radius * 2);

      const withUnit = (value:number) => `${value}${unit}`;

      const circlePositions:Record<CirclePos, [number, number]> = {
                "tl": [radius, radius],
                "tm": [fullWidth / 2, radius],
                "tr": [fullWidth - radius, radius],
                "bl": [radius, fullHeight - radius],
                "bm": [fullWidth / 2, fullHeight - radius],
                "br": [fullWidth - radius, fullHeight - radius],
                "ml": [radius, fullHeight  / 2],
                "mr": [fullWidth - radius, fullHeight  / 2],
      };
      const renderCircle = (pos:CirclePos) => {
          const [x, y] = circlePositions[pos];

          return svg`<circle class="circle ${pos}" cx="${withUnit(x)}" cy="${withUnit(y)}" r="${withUnit(radius)}" @click=${(evt:MouseEvent) => this.onCircleClick(pos, evt)} />`;
      }


      const circleIds:Array<CirclePos> = ["tl", "tm", "tr", "ml", "mr", "bl", "bm", "br"];

      const rotLineX = circlePositions["tm"][0];
      const rotLineY1 = circlePositions["tm"][1];
      const rotLineY2 = rotLineY1 - rotLineDistance; 

      return svg`
        <svg style="left: ${withUnit(-radius)}; top: ${withUnit(-radius)}" width="${withUnit(fullWidth)}" height="${withUnit(fullHeight)}">
        <rect id="rect" x="${withUnit(radius)}" y="${withUnit(radius)}" width="${withUnit(width)}" height="${withUnit(height)}" @click=${this.onRectClick} />
        ${circleIds.map(renderCircle)}
        </svg>
      ` 
      }
   */

  //<line id="rotLine" x1="${withUnit(rotLineX)}" y1="${withUnit(rotLineY1)}" x2="${withUnit(rotLineX)}" y2="${withUnit(rotLineY2)}" />
  //<line id="line" x1="" y1="218.10478688682443" x2="285.7142874703225" y2="193.40016804540556"></line>
  //<rect id="rect" x="${withUnit(radius)}" y="${withUnit(radius)}" width="${withUnit(width)}" height="${withUnit(height)}" @click=${this.onRectClick} />
  //${renderCircles()}
}
/*
        ${renderCircles([
            [0, 0],
            [width/2, height/2]
            ])}
 */
