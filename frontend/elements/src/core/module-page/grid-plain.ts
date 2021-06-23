import { LitElement, html, css, customElement, property} from 'lit-element';
import {nothing} from "lit-html";
import {BgBlue} from "@elements/_styles/bg";

@customElement('module-page-grid-plain')
export class _ extends BgBlue {

  static get styles() {
    return [...super.styles, css`
        :host {
            width: 100vw;
            height: 100vh;
            display: block;
        }

        #overlay {
            position: fixed;
            top: 0;
            left: 0; 
            display: block;
            z-index: 1000;
        }

        #grid {
            display: grid;

            grid-template-areas:
                        "sidebar header"
                        "sidebar main"
                        "sidebar footer";
            grid-template-columns: auto 1fr;
            grid-template-rows: auto 1fr auto;
            height: 100%;
            width: 100%;
        }

        aside {
            grid-area: sidebar;
        }

        header {
            grid-area: header;
        }

        main {
            display: flex;
            flex-direction: column;
            overflow: hidden;
            height: 100%;
            grid-area: main;
            overflow: auto;
        }

        footer {
            grid-area: footer;
        }
    `];
  }

  // Define the element's template
  render() {
    return html`
        <div id="grid">

            <aside><slot name="sidebar"></slot></aside>
            
            <header><slot name="header"></slot></header>

            <main>
                <slot name="main"></slot>
            </main>

            <footer><slot name="footer"></slot></footer>
        </div>
        <div id="overlay"><slot name="overlay"></slot></div>
    `;
  }
}
