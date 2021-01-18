import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';

@customElement('text-area')
export class _ extends LitElement {

  static get styles() {
    return [css`
    .wrapper{
        margin-bottom: 16px;
        border-radius: 14px;
        border: solid 1px #89b3ff;
        margin-top:40px;
        padding: 8px 16px;

    }
    label{
        padding-left: 8px;
    }
    span{
        color: #5590fc;
        
    }
    .textarea-wrapper{
        display:flex;
        align-items:center;
        
        position:relative;
    }

    input{
        outline:none;
        border:none;
        font-size:16px;
        padding:0 8px;
        width:100%;
    }
    focus{
        outline:none;
        border: solid 2px #5590fc;
    }
    wrapper:active{
        border: solid 2px #5590fc;
    }
    ::placeholder{
        color: #a1a8ad;
    }
    img{
        position:absolute;
        right:-10px;
    }
    textarea{
        resize:none;
        width:100%;
        outline:none;
        background: transparent;
        appearance: none;
        padding-left: 8px;
        font-family: Poppins;
        font-size:16px;
        border:none;
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
  error: boolean = false;

  render() {

    const {label} = this;




    return html`
       <div class="wrapper">
  <label for="name" class="">
    <span class="text-jibuttonBlue">${label}</span>
    <div class="textarea-wrapper">
      <textarea rows="10"
       type="text" placeholder="Jane Doe" ></textarea>
    </div>
  </label>
</div>
   
     
  `;
  }
}