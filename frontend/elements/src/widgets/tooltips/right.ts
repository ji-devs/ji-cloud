import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from 'lit-html';

export type Kind = "plain" | "error" | "success";
export type Padding = "small" | "large";

@customElement('tooltip-right')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .tooltip-wrapper{
      margin-left:20px;
    }
    .tooltip-wrapper .tooltip-wrapper-inside {
        visibility: visible;
        
        text-align: center;
        border-radius: 12px;
        position: absolute;
        z-index: 1;
        box-shadow: 2px 2px 6px 0 rgba(0, 0, 0, 0.16);
        top:10px;
        
    
      }
      .success .tooltip-wrapper-inside, .error .tooltip-wrapper-inside {
        width: 295px;
        height: 69px;
      }
    
      .plain .tooltip-wrapper-inside{
        padding-left:16px;
        padding-right:16px;
        height:48px;
        

      }
      
      .tooltip-wrapper .tooltip-wrapper-inside::after {
        content: "";
        position: absolute;
        top: 50%;
        right: 100%;
        margin-top: -5px;
        border-width: 5px;
        border-style: solid;
        
      }
    
      .tooltip-wrapper .tooltip-wrapper-inside::before {
        content: "";
        position: absolute;
        top: 50%;
        right: 100%;
        margin-top: -6px;
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
      .success .tooltip-wrapper-inside{
        background-color: #f0fcf5;
        color: #42cc7a;
        border: solid 1px #42cc7a;
      }
      .plain .tooltip-wrapper-inside{
        background-color:#2040a4;
        color: #fffff;
        border: none;
      }
    
      .error .tooltip-wrapper-inside::before {
        border-color: transparent #f00813 transparent  transparent;
    
      }
    
      .plain .tooltip-wrapper-inside::before {
        border-color: transparent transparent transparent  transparent;
    
      }
    
      .plain .tooltip-wrapper-inside::after{
        border-color: transparent #2040a4 transparent transparent;
      }

      .error .tooltip-wrapper-inside::after{
        border-color: transparent #fff4f4 transparent transparent;
      }
      .success .tooltip-wrapper-inside::before{
        border-color: transparent #42cc7a transparent  transparent;
      }
      .success .tooltip-wrapper-inside::after{
        border-color: transparent  #f0fcf5 transparent  transparent;
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
      : "success" ? "success" 
      : "plain" ? "plain" 
      : "nothing"
      ;

    const uiPath = kind === "error" ? "group-12812.svg" : "success" ? "green-check.svg"
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