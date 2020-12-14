import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('img-thumb')
export class _ extends LitElement {
  @property()
  src:string = ""; 

  render() {
    const { src} = this;

    return html`<img src="${src}"></img>`;
  }
}