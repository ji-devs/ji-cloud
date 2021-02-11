import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('menu-play')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
        display:flex;
        align-items:center;
    }
    .player{
        width: 222px;
        height: 48px;
        background-color:#ffffff;
        border-radius:24px;
        display:flex;
        box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
        align-items:center;
        justify-content:center;
        margin-right:24px;
    }
    p{
        margin:0 0 0 8px;
        
    }
    .arrows{
        border-right:solid 1px #606060;
        padding-right:16px;
        margin-right:16px;
    }

 
    `];
  }



  render() {

    const {} = this;
    const STR_PREVIEW = "Preview"
    return html`
    <main>
    <div class="player">
        <div class="arrows">
            <img-ui path="icn-undo.svg"></img-ui>
            <img-ui path="icn-redo.svg"></img-ui>
        </div>
    <img-ui path="Icn_Play_Blue.svg"></img-ui>
    <p>${STR_PREVIEW}</p>
    </div>
    <img-ui path="jiggling.png"></img-ui>
   
    </main>
  `;
  }
}