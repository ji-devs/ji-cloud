import { LitElement, html, css, customElement, property} from 'lit-element';
import {nothing} from "lit-html";
import {BgBlue} from "@elements/_styles/bg";
import { loadAllFonts, loadFonts } from '@elements/_themes/themes';
import { classMap } from 'lit-html/directives/class-map';
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

        .hidden {
            display: none;
        }
    `];
  }

  @property({type: Boolean})
  fontsLoaded:boolean = false;

  @property()
  fontFamilies:Array<string> | undefined;

  firstUpdated(_:any) {
      if(this.fontFamilies) {
          loadFonts(this.fontFamilies).then(() => {
            this.fontsLoaded = true;
          });
      } else {
        loadAllFonts().then(() => {
            this.fontsLoaded = true;
        });
    }
  }

  // Define the element's template
  render() {
      const classes = classMap({hidden: !this.fontsLoaded});
    return html`
        <div id="grid" class=${classes}>

            <aside id="sidebar"><slot name="sidebar"></slot></aside>
            
            <header id="header"><slot name="header"></slot></header>

            <main id="main">
                <slot name="main"></slot>
            </main>

            <footer id="footer"><slot name="footer"></slot></footer>
        </div>
        <div id="overlay" class=${classes}><slot name="overlay"></slot></div>
    `;
  }
}
