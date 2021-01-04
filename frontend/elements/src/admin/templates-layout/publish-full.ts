import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('publish-full')
export class _ extends LitElement {
  static get styles() {
    return [css`
        main{
            background-color:red;
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
            padding:56px 64px;
            width:100%;
        }
        .content{
            display:flex;
            
           
        }
       .column{
           width:25%;
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
            <h1>${title}</h1>
            <p name="subtitle">${subtitle}</p>
            <div class="content">
                <div class="column"><slot name="column_one"></slot></div>
                <div class="column"><slot name="column_two"></slot></div>
                <div class="column"><slot name="column_three"></slot></div>
                <div class="column"><slot name="column_four"></slot></div>
            
            </div>
            <slot name="button"></slot>
            </div>
        </div>
    </main>

  `;
  }
}