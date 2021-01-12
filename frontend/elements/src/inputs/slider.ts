import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('slider-checkbox')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
        display:flex;
        align-items:center;
    }
    .onoffswitch {
        position: relative; 
        width: 40px;
        -webkit-user-select:none; 
        -moz-user-select:none; 
        -ms-user-select: none;
    }
    .onoffswitch-checkbox {
        position: absolute;
        opacity: 0;
        pointer-events: none;
    }
    .onoffswitch-label {
        display: block; 
        overflow: hidden; 
        cursor: pointer;
        border-radius: 10px;
        height:20px;
        background-color: #d3ddea;
    }
    .onoffswitch-inner {
        display: block; 
        width: 200%; 
        margin-left: -100%;
        transition: margin 0.3s ease-in 0s;
    }
    .onoffswitch-inner:before, .onoffswitch-inner:after {
        display: block; 
        float: left; 
        width: 50%; 
        height: 20px; 
        padding: 0; 
        line-height: 20px;
        font-size: 14px; 
        box-sizing: border-box;
    }
 

    .onoffswitch-switch {
        display: block; 
        width: 20px;
        height:20px; 
        margin: 0px;
        background: #A1A1A1;
        position: absolute; top: 0; bottom: 0;
        right: 20px;
        border-radius: 50%;
        transition: all 0.3s ease-in 0s; 
    }
    .onoffswitch-checkbox:checked + .onoffswitch-label .onoffswitch-inner {
        margin-left: 0;
    }
    .onoffswitch-checkbox:checked + .onoffswitch-label .onoffswitch-switch {
        right: 0px; 
        background-color: #5590FC; 
    }
    p{
        margin-right:66px;
    }
    `];
  }



  @property()
  label:string = ""; 

  render() {

    const {label} = this;

    return html`
    <div class="wrapper">
        <p>${label}</p>
        <div class="onoffswitch">
    <input type="checkbox" name="onoffswitch" class="onoffswitch-checkbox" id="myonoffswitch" tabindex="0" checked>
    <label class="onoffswitch-label" for="myonoffswitch">
        <span class="onoffswitch-inner"></span>
        <span class="onoffswitch-switch"></span>
    </label>
</div>
    </div>
  `;
  }
}