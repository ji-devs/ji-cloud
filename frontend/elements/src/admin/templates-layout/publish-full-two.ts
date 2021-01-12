import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('publish-full-two')
export class _ extends LitElement {
  static get styles() {
    return [css`
        main{
           
            background-image:url("http://localhost:4102/ui/Background@2x.jpg");
            background-repeat: no-repeat;
            background-attachment: inherit;
            background-position: top;
            height:100vh;
            display:flex;
            align-items:center;
        }
        .wrapper{
            background-color: #ffffff;
            width: 1474px;
            height: 802px;
            border-radius: 32px;
            display:flex;
            align-items:center;
            margin-right:auto;
            margin-left: auto;
           
            
            
        }
        .inside-wrapper{
            padding:56px 64px;
            width:100%;
            display:flex;
            align-items:center;
            flex-direction:column;
        }

   
       h1{
        font-size: 32px;
        font-weight: 900;
        color: #ff6639;
       }
       p{
        font-weight: 500;
        color: #4a4a4a;
       }
       ::slotted([slot="button-collection"]){
        display:flex;
        justify-content:center;
        align-items:center;
        margin-top:64px;
        margin-right:32px;
    }
    ::slotted([slot="button-collection"]:last-child){
       margin-right:0;
    }
    .button-wrapper{
        display:flex;
    }
       
   
    `];
  }

  @property()
  title:string = ""; 

  @property()
  subtitle:string = ""; 

  render() {

    const {title, subtitle} = this;

    return html`    
    <main>
        <div class="wrapper">
            
            <div class="inside-wrapper">
                <slot name="animation"></slot>
                <h1>${title}</h1>
                <p name="subtitle">${subtitle}</p>
                <div class="button-wrapper">
                    <slot name="button-collection"></slot>
                </div>
            </div>
        </div>
    </main>

  `;
  }
}