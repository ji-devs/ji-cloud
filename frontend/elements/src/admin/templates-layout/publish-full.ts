import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('publish-full')
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
            margin-right:auto;
            margin-left: auto;
            flex-direction:column;
            
            
        }
        .inside-wrapper{
            padding:26px 64px;
            width:100%;
        }
        .content{
            display:flex;
            
           
        }
       .column{
           width:25%;
           margin-right:48px;
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
       ::slotted([slot="button"]){
           display:flex;
           justify-content:center;
           margin-top:20px;
       }
       .button-wrapper{
           position:relative;
       }
       ::slotted([slot="buttoncollection"]){
        display:flex;
        justify-content:center;
        align-items:center;
    }
    ::slotted([slot="column_one"]){
        display:flex;
        flex-direction:column;
    
        align-items:center;
    }
    ::slotted(*){
        position:relative;
    }
    ::slotted([slot="tooltip"]){
        position:absolute;
        top:56px;
        left:40%;
    
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
            <slot name="animation"></slot>
            <div class="inside-wrapper">
            
                <h1>${title}</h1>
                <p name="subtitle">${subtitle}</p>
                <div class="content">
                    <div class="column"><slot name="column_one"></slot></div>
                    <div class="column"><slot name="column_two"></slot></div>
                    <div class="column"><slot name="column_three"></slot></div>
                    <div class="column"><slot name="column_four"></slot></div>
                </div>
                <div class="button-wrapper">
                    <slot name="button"></slot>
                    <slot name="tooltip"></slot>
                </div>
                <slot name="button-collection"></slot>
            </div>
        </div>
    </main>

  `;
  }
}