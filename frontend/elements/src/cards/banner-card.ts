import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
export type Color = "green" | "blue" | "white";
@customElement('banner-card')
export class _ extends LitElement {

  static get styles() {
    return [css`
    .wrapper{
        width: 354px;
        height: 40px;
        box-shadow: 2px 3px 2px 0 rgba(208, 211, 232, 0.5);
        display:flex;
        align-items:center;
        justify-content:center;
        border-radius:0 0 20px 20px;
    }
    .green {
        background-color:#dffada;
    }
    .blue{
        background-color:#e7f0fe
    }
    .white{
        background-color:#ffffff;
    }
    img-ui {
        margin-right: 8px;
    }
    p{
       color: #5590fc;
       font-weight:400;
    }
    .imghidden, .teamhidden{
        display:none;
    }
    .team{
        font-weight:500
    }
    `];
  }

@property()
icon: string = "";

@property()
label: string = "";

@property()
team: string = "";

@property({type:Boolean})
imghidden: boolean = false;

@property({type:Boolean})
teamhidden: boolean = false;

@property()
color: Color = "blue";



  render() {
    const {icon, label, color, imghidden,team, teamhidden} = this;
   

    return html`
<div class="wrapper ${color}">
    
        <img-ui path="${icon}" class="${imghidden ? 'imghidden' : ''}"></img-ui>
        <p class="team ${teamhidden ? 'teamhidden' : ''}">${team}</p>
        <p>${label}</p>
    
</div>
  `;
  }
}