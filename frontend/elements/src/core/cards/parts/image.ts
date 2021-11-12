import { LitElement, html, css, customElement, property } from "lit-element";
@customElement("search-image")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
        .image-wrapper{
            height:200px;
            width:354px;
            overflow:hidden;
        }
        img-ui{
            height:100%;
            width:100%;
            
        }
    
      }`,
        ];
    }

    @property()
    image: string = "";

    render() {
        const { image } = this;

        return html`
            <div class="image-wrapper">
                <img-ui path="${image}"></img-ui>
            </div>
        `;
    }
}
