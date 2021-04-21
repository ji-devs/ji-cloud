import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import { styleMap } from 'lit-html/directives/style-map';

@customElement('card-text')
export class _ extends LitElement {
  static get styles() {
      return [css`

    `];
  }

  @property()
  value:string = "";

  @property()
  fontFamily:string = "";

  @property()
  color:string = "";

  @property()
  fontSize:string = "";

  render() {
    const { fontFamily, color, fontSize, value} = this;

    let style:any = {};

    if(fontFamily !== "") {
        style.fontFamily = fontFamily;
    }

    if(color !== "") {
        style.color = color;
    }

    if(fontSize !== "") {
        style.fontSize = fontSize;
    }

    style = styleMap(style);
    return html`<span style=${style}>${value}</span>`
  }
}
