import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing} from 'lit-html';

export type Color = "green" | "blue" | "white";
export type IconKind = "ji" | "";

@customElement("card-banner")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
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
            height:19px;
            width:auto;
        }
        p{
        color: #5590fc;
        font-weight:400;
        }
    
        .team{
            font-weight:500
        }
    `,
    ];
  }

 
  @property()
  label: string = "";

  @property()
  team: IconKind = "ji";

  @property()
  color: Color = "blue";

  @property()
  kind: IconKind = "ji";


  render() {
    const {label, color, kind } = this;
    const iconPath = kind === "ji" ? "JI_Badge.svg"
        : nothing;
    const teamtitle = kind === "ji" ? "Ji Team -  "
        : nothing;


    return html`
      <div class="wrapper ${color}">
        <img-ui path="${iconPath}"></img-ui>
        <p class="team">${teamtitle}</p>
        <p>${label}</p>
      </div>
        
    `;
  }
}
