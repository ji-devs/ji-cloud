import { mediaUi } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/entry/jig/publish/publish-placeholder-img"; 

@customElement('publish-page-two')
export class _ extends LitElement {
  static get styles() {
    return [css`
        main{
            background-repeat: no-repeat;
            background-attachment: inherit;
            background-position: top;
            height:100vh;
            display:flex;
            align-items:center;
        }
        .wrapper{
            background-color: #ffffff;
            width: 1688px;
            height: 802px;
            border-radius: 32px;
            display:flex;
            align-items:center;
            margin-right:auto;
            margin-left: auto;
           
        }
        .inside-wrapper{
            padding-top:56px;
            margin-left:auto;
            margin-right:auto;
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
        z-index:1
        
    }
    ::slotted([slot="dropdown"]){
        position:absolute;
        z-index:10;
        top: -150px;
        left: 185px;
    }
    ::slotted([slot="button-collection"]:last-child){
       margin-right:0;
    }
    .button-wrapper{
        display:flex;
        position:relative;
        
    }
    .closed{
        display:none;
    }
       
   
    `];
  }

  @property({type:Boolean})
  closed: boolean = false;
  
  render() {

    const {closed} = this;
    
    const STR_TITLE = "Your JIG is on air now";
    const STR_SUBTITLE ="What would you like to do next?"

    return html`    
    <main style="background-image:url('${mediaUi("Background@2x.jpg")}')">
        <div class="wrapper">
            
            <div class="inside-wrapper">
            <publish-placeholder-img class="animation"></publish-placeholder-img>
                <h1>${STR_TITLE}</h1>
                <p name="subtitle">${STR_SUBTITLE}</p>
                <div class="button-wrapper">
                    <slot name="button-collection"></slot>
                    
                    <slot name="dropdown" class="${closed ? 'closed' : ''}"></slot>
                    
                </div>
            </div>
        </div>
    </main>

  `;
  }
}