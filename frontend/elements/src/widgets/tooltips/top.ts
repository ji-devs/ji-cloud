import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from 'lit-html';

export type Kind = "plain" | "error"

@customElement('tooltip-top')
export class _ extends LitElement {
  static get styles() {
    return [css`
  
    .tooltip-wrapper .tooltip-wrapper-inside {
        visibility: visible;
        width: 295px;
        height: 69px;
        text-align: center;
        border-radius: 12px;
        position: absolute;
        z-index: 1;
        box-shadow: 2px 2px 6px 0 rgba(0, 0, 0, 0.16);
        top:10px;
    
      }
      
      .tooltip-wrapper .tooltip-wrapper-inside::after {
        content: "";
        position: absolute;
        bottom: 100%;
        left: 50%;
        margin-left: -5px;
        border-width: 5px;
        border-style: solid;
      }
    
      .tooltip-wrapper .tooltip-wrapper-inside::before {
        content: "";
        position: absolute;
        bottom: 100%;
        left: 50%;
        margin-left: -6px;
        border-width: 6px;
        border-style: solid;
      }
    
      .tooltip-wrapper-inside{
          display:flex;
          justify-content:center;
          align-items:center;
      }
      img-ui{
        margin-right:16px;
      }
      .error .tooltip-wrapper-inside{
        background-color: #fff4f4;
        color: #ed464e;
        border: solid 1px #f00813;
      }
    
      .error .tooltip-wrapper-inside::before {
        border-color: transparent transparent #f00813 transparent;
    
      }
    
      .error .tooltip-wrapper-inside::after{
        border-color: transparent transparent #fff4f4 transparent;
      }
      p{
          padding:0;
          margin:0;
      }
        
   
    `];
  }

  @property()
  kind:Kind = "error";

  render() {

    const {hidden, kind} = this;

    const wrapperClass = kind === "error" ? "error"
      : "";

    const uiPath = kind === "error" ? "group-12812.svg"
      : "";

    return html`
   
        <div class="tooltip-wrapper ${wrapperClass}">
            <span class="tooltip-wrapper-inside">
                ${kind !== "plain"
                  ? html`<img-ui path="${uiPath}" alt=""></img-ui>`
                  : nothing
                }
                <p><slot></slot></p>
            </span>
        
        </div>
    
  `;
  }
}