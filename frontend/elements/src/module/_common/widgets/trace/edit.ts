import { LitElement, svg, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

type TraceEditKind = "path" | "rect" | "circle";

@customElement('trace-edit')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              position: absolute;
              top: 0;
              left: 0;
          }

          #fillRect {
              fill: black;
              fill-opacity: 0.5;
          }
    `]
  }

  @property({type: Number})
  canvasWidth:number = 0;

  @property({type: Number})
  canvasHeight:number = 0;

  @property({type: Array})
  kind:TraceEditKind = "path";

  @property({type: Array})
  path:Array<[number, number]> = [];

  @property({type: Array})
  rect:[number, number, number, number]= [0, 0, 0, 0];

  @property({type: Array})
  circle:[number, number, number]= [0, 0, 0];


  render() {
      const {canvasWidth, canvasHeight, kind, path, rect, circle} = this;
      return svg`
        <svg width="${canvasWidth}px" height="${canvasHeight}px">
        <rect id="fillRect" x="0" y="0" width="${canvasWidth}px" height="${canvasHeight}px" />
        <slot></slot>
        ${kind === "path" ? renderPath(path)
            : kind === "rect" ? renderRect(rect)
            : kind === "circle" ? renderCircle(circle)
            : nothing
        }
        </svg>
        `;
  }
}

function renderPath(path: Array<[number, number]>) {
    console.log(path);
}
function renderRect([x, y, width, height]:[number, number, number, number]) {
    console.log(x, y, width, height);
}
function renderCircle([x, y, radius]:[number, number, number]) {
    console.log(x, y, radius);
}
