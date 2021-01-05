import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('signup-full-wide')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
        display:flex;
    
    }
    .inside-wrapper{
        width:624px;
    }
    .side-image{
        width: 480px;
        min-width:300;
        height: 100vh;
        background-color: #def4ff;
        background-image: url('https://i.ibb.co/g9N7MLy/shapes-1.png');
        background-repeat: no-repeat;
        background-attachment: inherit;
        background-position: center;
    }
    .content-wrapper{
        padding:80px;
        width:867px;
    }
    h1{
        font-size: 32px;
        font-weight: 900;
        color:#5662a3
    }
    
   
   
    `];
  }

  @property()
  title:string = ""; 

  @property()
  subtitle:string = ""; 

  @property()
  hidden:boolean = true; 

  render() {

    const {title, hidden, subtitle} = this;

    return html`
 <div class="wrapper">
  <div class="side-image">
  </div>
  <div class="content-wrapper">
    <h1>${title}</h1>
    <slot name="subtitle"></slot>
    <slot name="main"></slot>
    <slot name="submit"></slot>
  </div>
</div>
  `;
  }
}