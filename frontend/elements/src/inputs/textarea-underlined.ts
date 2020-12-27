import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';

@customElement('textarea-underlined')
export class _ extends LitElement {

  static get styles() {
    return [css`
    .wrapper{
        margin-bottom: 16px
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
        border-bottom:solid 1px #e5e7ef;
        width:100%;
        outline:none;
        background: transparent;
        appearance: none;
        border-right: none;
        border-top: none;
        border-left: none;
        padding-left: 8px;
        font-family: Poppins;
        font-size:16px;
    }
   
  
    `];
  }

  @property()
  label: string = "";
  @property()
  src: string = "";
  @property()
  icon: boolean = false;


  render() {

    const {label} = this;

    return html`
    
    <div class="wrapper">
  <label for="name" class="">
    <span class="text-jibuttonBlue">${label}</span>
    <div class="textarea-wrapper">
      <textarea data-id="{{inputDataId}}" rows="10"
        contenteditable="true" type="text" placeholder="Jane Doe" aria-label="Full name"></textarea>
    </div>
  </label>
</div>
     
  `;
  }
}