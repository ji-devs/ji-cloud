import { LitElement, html, css, customElement, property } from "lit-element";

export type IconKind = "age" | "lang" | "time" | "file" | "jiwhite";

@customElement("tag-icon")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
    p{
        font-size: 14px;
        font-weight: 500;
        color:#798b96;
        margin-left:8px;
        margin-top:0;
        margin-bottom:0;
  
    }
    .wrapper{
        display:flex;
        align-items:center;
        height:20px;
    }
    .darkgrey {
        color:#798b96;
    }
    .lightblue{
        color:#afcbf4;
    }
    .white {
        color:#ffffff;
    }
    `,
        ];
    }

    @property()
    kind: IconKind = "age";

    @property()
    label: string = "";

    render() {
        const { kind, label } = this;

        const iconPath =
            kind === "age"
                ? "Icn_Age.svg"
                : kind === "lang"
                ? "globe.svg"
                : kind === "time"
                ? "Icn_clock.svg"
                : kind === "file"
                ? "icn-file.svg"
                : kind === "jiwhite"
                ? "JI_Logo_White.svg"
                : "nothing";

        const colorClass =
            kind === "age"
                ? "darkgrey"
                : kind === "lang"
                ? "darkgrey"
                : kind === "time"
                ? "lightblue"
                : kind === "file"
                ? "lightblue"
                : kind === "jiwhite"
                ? "white"
                : "nothing";

        return html`
            <div class="wrapper">
                <img-ui path="${iconPath}"></img-ui>
                <p class="${colorClass}">${label}</p>
            </div>
              
        `;
    }
}
