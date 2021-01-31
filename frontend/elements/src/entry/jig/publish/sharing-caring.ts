import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/variants/title-w-icon";
@customElement('sharing-caring')
export class _ extends LitElement {
  static get styles() {
    return [css`
        main{
            width: 410px;
            height: 440px;
            border-radius:12px;
            background-color: #ffe1d5;
            position:relative;
            display:none;

        }
        div {
            padding:32px
        }
        .sharing{
            display:block;
        }
        .close{
            position:absolute;
            top:10px;
            right:10px;
        }
        h1{
            font-size:32px;
            font-weight:900;
            color:#fd6220;
            margin-bottom:16px;
            margin-top:0
        }
        p{
            margin-bottom: 32px;
        }
    
    `];
  }


  @property({type:Boolean})
  sharing: boolean = false;
  

  render() {

    const {sharing} = this;
    const STR_TITLE = "Sharing is Caring";
    const STR_SUB = "Imagine how great it is to have a huge collection of great activities made by teachers, and everything you need, you can find there.";
    const STR_TOGGLE = "Set the toggle back to share your JIG with the JI community."
    
    return html`
    <main class="${sharing ? 'sharing' : ''}">
    <div>
       <img-ui class="close" path="icn-x-close.svg"></img-ui>
       <h1>${STR_TITLE}</h1>
       <p>${STR_SUB}</p>
       <p> ${STR_TOGGLE}</p>
       </div>
    </main>
  `;
  }
}