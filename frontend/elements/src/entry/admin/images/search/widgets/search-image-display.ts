import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from 'lit-html';
export type Mode = "saved" | "published" ;
@customElement('search-image-display')


export class _ extends LitElement {
    static get styles() {
        return [css`
    main{
        display:flex;
        flex-direction:column;
        align-items:center;
        width:260px;
    }
    main .image-wrapper{
        border:solid 4px #ffffff;
        border-radius:8px;
        width: 254px;
        height:190px;
       
    }
    img-ui{
        width:100%
        height:auto;
        margin-left:-10px;
    }
    .active .image-wrapper{
        border-color:#5590fc
    }
    .active img-ui{
        margin-left:0;
    }
    p::before{
        content: '';
        height:16px;
        width:16px;
        border-radius:50%;
        display:inline-block;
        position:absolute;
        top: 20px;
        left: 0;
        
    }
    .published::before{
        background-color: #6eca90;
    }
    .saved::before{
       background-color: #e36486;

    }
    .text{
        position:relative;
        width:100%;
       
    }
    p{
        display:flex;
        justify-content:center;
    }
    .active p{
        color:#5590fc
    }
   
    `];
    }

    @property()
    thumbnail: string = "";

    @property()
    imagename: string = "";

    @property({ type: Boolean })
    active: boolean = false;

    @property()
    mode: Mode = "saved";

    render() {
        const { thumbnail, active,imagename, mode } = this;
        const color = mode === "saved" ? "saved"
        : mode === "published" ? "published"
        : nothing;


        return html`
<main class="${active ? " active" : "" }">
    <div class="image-wrapper">
        <img-ui path="${thumbnail}"></img-ui>
    </div>
    <div class="text">
    <p class="${color}">${imagename}</p>
    </div>
</main>
  `;
    }
}