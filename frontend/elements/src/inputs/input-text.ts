import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';

@customElement('input-text')
export class _ extends LitElement {

  static get styles() {
    return [css`
   .input-wrapper{
    position:relative;
    width:inherit;
    height:64px;
    border: solid 1px #89b3ff;
    border-radius:14px;
    padding: 8px 48px 8px 16px;
   }
   .errorwrapper {
    border: solid 1px #f00813;
    background-color: #fff4f4;
   }
   .errorwrapper input {
    background-color: #fff4f4;
   }
   .instruction{
     display:none;
     margin-left: 8px;
   }
   input{
    outline:none;
    border:none;
    margin-top: 33px;
    width: inherit;
   }
   label{
    position:absolute;
    top: 0;
    left: 0;
    font-size: 16px;
    padding: 8px 0px 0px 16px;
    color: #5590fc;
   }
   .input-wrapper:active{
    border: solid 2px #5590fc;
   }
   input{ font-size:16px;}
   .helpertext{
     font-size: 14px;
     color: #5590fc;
     margin-top:4px;
     font-weight: 500;
   }
   active .helpertext{
    display:block;
   }
   .error{
    font-size: 14px;
    color: #f00813;
    margin-top:4px;
    font-weight: 500;
    padding-left:8px;
    display:block;
   }
   p{display:none;}
   .imghidden {
     display:none;
   }
   img-ui{
    position: absolute;
    top: 33%;
    right: 12px;
   }
   .visible{
     display:block;
   }
   .hidepassword{
     display:block;
   }
   
  
    `];
  }

  @property()
  label: string = "";

  @property()
  errormessage: string = "";

  @property()
  instruction: boolean = false;

  @property()
  helpertext: string = "";

  @property()
  placeholder: string = "";

  @property()
  path: string = "";

  @property({type: Boolean})
  error: boolean = false;

  @property({type: Boolean})
  errorwrapper: boolean = false;

  @property({type: Boolean})
  imghidden: boolean = false;

  @property({type: Boolean})
  visiblepassword: boolean = false;
  
  @property({type: Boolean})
  hidepassword: boolean = true;

  render() {

    const {label, helpertext, error, instruction, errormessage, placeholder, errorwrapper, path,pathtwo, imghidden, visiblepassword,hidepassword} = this;

    const errorClasses = classMap({ 
      error,
    });

    const instructionClasses = classMap({ 
      instruction,
    });


    return html`
    
    <div class="input-wrapper ${errorwrapper ? 'errorwrapper' : ''}">
        <input placeholder="${placeholder}" type="text" class="">
        <label class="">${label}</label>
        <img-ui path="${path}" class="${imghidden ? 'imghidden' : ''} ${visiblepassword ? 'visible' : 'imghidden'}"></img-ui>
        <img-ui path="${pathtwo}" class="${hidepassword ? 'imghidden' : 'hidepassword'}"></img-ui>

    </div>
    <p class="${instructionClasses}">${helpertext}</p>
    <p class="${errorClasses}">${errormessage}</p>
     
  `;
  }
}