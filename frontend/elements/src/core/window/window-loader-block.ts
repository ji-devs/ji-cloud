import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";

@customElement('window-loader-block')
export class _ extends LitElement {
  static get styles() {
    return [css`
        main {
            position: absolute;
            top: 0;
            left: 0;
            width: 100vw;
            height: 100vh;
            background-color: rgba(255, 255, 255, .7);
            color: grey;
        }
    `]
  }

  @property({type: Boolean})
  visible:boolean = false;

  render() {
      const {visible} = this;
      return visible ? html`<main>Loading...</main>` : nothing;
  }
}

 