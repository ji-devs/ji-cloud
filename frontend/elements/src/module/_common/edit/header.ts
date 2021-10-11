import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import "@elements/_bundles/_sub-bundles/overlay"

@customElement('module-header')
export class _ extends LitElement {
  static get styles() {
      return [css`
          .topRight {
              position: absolute;
              top: 21px;
              right: 40px;
              display: flex;
              gap: 24px;
              align-items: center;
          }
          .title {
              margin-top: 90px;
              font-size: 32px;
              font-weight: 900;
              letter-spacing: -0.32px;
              text-align: left;
              color: var(--dark-blue-4);
          }

          #gear-img {
              cursor: pointer;
          } 
    `];
  }

  @property()
  headerTitle:string = "";

  @property()
  tooltipTitle:string = "";

  @property()
  tooltipBody:string = "";

  imgRef:HTMLElement | undefined;

  //instead of firstUpdated since tooltip needs the size of the image to position correctly
  onImageLoaded() {
      this.imgRef = this.shadowRoot?.getElementById("gear-img") as HTMLElement;
      this.requestUpdate();
  }

  onGearClick() {
      const tooltipRef = this.shadowRoot?.getElementById("tooltip");

      (tooltipRef as any).selfClosed = false;
  }

  render() {
      const {imgRef, headerTitle, tooltipBody, tooltipTitle} = this;

      return html`
          <section>
                  <div class="topRight">
                      <slot name="controller"></slot>
                      <img-ui @click=${this.onGearClick} @image-load=${this.onImageLoaded} id="gear-img" path="module/_common/edit/header/jiggling-gear.png"></img-ui>
                      ${imgRef ? renderTooltip(tooltipTitle, tooltipBody, imgRef) : nothing} 
                  </div>
                  <div class="title">${headerTitle}</div>
                  <slot></slot>
          </section>
      `
  }
}

function renderTooltip(title:string, body:string, targetRef:HTMLElement) {
    const marginX = -33; 

    const showId = `module-header`;
    return html`<overlay-container>
        <overlay-tooltip-info id="tooltip" .target=${targetRef} .marginX=${marginX} targetAnchor="bm" contentAnchor="tr" title="${title}" body="${body}" showId="${showId}" closeable></overlay-tooltip-info>
</overlay-container>
    `
            
}
