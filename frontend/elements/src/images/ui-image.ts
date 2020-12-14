import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import {mediaUi} from "@utils/path";

@customElement('ui-image')
export class _ extends LitElement {
  @property()
  path:string = ""; 

  // Define the element's template
  render() {
    const { path } = this;

    const src = mediaUi(path);

    return html`<img src="${src}"></img>`;
  }
}