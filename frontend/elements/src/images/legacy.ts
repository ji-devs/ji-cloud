import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import {legacyMock, legacyMedia} from "@utils/path";
import "./basic";

@customElement('img-legacy')
export class _ extends LitElement {
  @property()
  jigId:string = ""; 

  @property()
  moduleId:string = ""; 

  @property()
  path:string = ""; 

  @property({type: Boolean})
  mock:boolean = false;

  render() {
    const { jigId, moduleId, path, mock } = this;

    const src = mock 
        ? legacyMock({jigId, moduleId, path}) 
        : legacyMedia ({jigId, moduleId, path}) 

    return html`<img-basic src="${src}"></img>`;
  }
}