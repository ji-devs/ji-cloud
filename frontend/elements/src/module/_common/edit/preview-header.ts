import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {ModuleKind, STR_MODULE_PREVIEW_TOOLTIP_BODY} from "@elements/module/_common/types";

const STR_TITLE = "Preview Mode";

const STR_TOOLTIP_CONTINUE = "Click to continue";
const STR_TOOLTIP_GETTING_STARTED = "Time to play!";

@customElement('module-preview-header')
export class _ extends LitElement {
  static get styles() {
      return [css`
          section {
              display: flex;
              width: 100%;
              height: 112px;
              box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
              background-color: var(--white);
              justify-content: space-between;
              align-items: center;
          }

          .nav {
            /* same as sidebar body */
            padding-left: 50px;
            padding-right: 50px;
            width: 456px;

            padding-top: 10px;
          }
          .btn, .nav {
              z-index: 1;
          }
          .title {
              position: absolute;
              height: 112px;
              line-height: 112px;
              left: 0;
              top:0;
              font-size: 28px;
              text-align: left;
              color: var(--dark-blue-4);
              width: 100%;
              text-align: center;
          }
    `];
  }


  firstUpdated(_changed:any) {
      this.sectionRef = this.shadowRoot?.getElementById("section") as HTMLElement;
      this.requestUpdate();
  }

  @property()
  moduleKind:ModuleKind = "memory";

  @property({type: Boolean})
  continueTooltip:boolean = false;

  sectionRef:HTMLElement | undefined;

  render() {
      const {sectionRef, moduleKind, continueTooltip} = this;
      return html`
          <section id="section">
              <div class="title">${STR_TITLE}</div>
              <div class="nav">
                  <slot name="nav"></slot>
              </div>
              <div class="btn"><slot name="btn"></slot></div>
           </section>
         ${sectionRef ? renderIntroTooltip(moduleKind, sectionRef) : nothing} 
         ${sectionRef && continueTooltip ? renderContinueTooltip(sectionRef) : nothing} 

      `
  }
}

function renderIntroTooltip(moduleKind:ModuleKind, targetRef:HTMLElement) {
    const body = STR_MODULE_PREVIEW_TOOLTIP_BODY[moduleKind];
    if(!body) {
        return nothing;
    }

    const showId = `preview-header-intro-${moduleKind}`;

    return html`<overlay-container>
        <overlay-tooltip-info 
            id="tooltip" 
            .target=${targetRef} 
            targetAnchor="bl" 
            contentAnchor="tl" 
            title="${STR_TOOLTIP_GETTING_STARTED}" 
            body="${body}" 
            showId="${showId}" 
            closeable>
       </overlay-tooltip-info>
    </overlay-container>`
}
function renderContinueTooltip(targetRef:HTMLElement) {
    return html`<overlay-container>
        <overlay-tooltip-info 
            id="tooltip" 
            .target=${targetRef} 
            targetAnchor="br" 
            contentAnchor="tr" 
            title="" 
            body="${STR_TOOLTIP_CONTINUE}" 
            closeable>
       </overlay-tooltip-info>
    </overlay-container>`
}
