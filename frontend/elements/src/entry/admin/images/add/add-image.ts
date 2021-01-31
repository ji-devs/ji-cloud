import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('add-image')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
        width: 288px;
        height: 216px;
        border-radius: 4px;
        background-color: #e6f0ff;
        display:flex;
        align-items:center;
       
    }
    .inside-wrapper{
        width: 256px;
        height: 184px;
        border: dashed 3px #5590fc;
        margin-left:auto;
        margin-right:auto;
        display:flex;
        align-items:center;
        justify-content: center;

    }
    .wrapper:hover .inside-wrapper{
        border-color:#2b54b8;
    }
    .inside-wrapper {
        transition-duration: 200ms;
        transition-timing-function: ease-in-out;
    }
    `];
  }



  @property()
  label:string = ""; 

  render() {

    const STR_UPLOAD = "Upload image";

    return html`
 
  <div class="wrapper">
    <div class="inside-wrapper">
        <button-rect color="blue" size="medium" iconBefore="plus">${STR_UPLOAD}</button-rect>
    </div>
  </div>
  
    
 

  <div>
    <p>Select an image kind:</p>

    <div>
      <input type="radio" id="sticker" name="img_kind" value="sticker" checked>
      <label for="sticker">Sticker</label>
    </div>
    <div>
      <input type="radio" id="canvas" name="img_kind" value="canvas">
      <label for="canvas">Canvas</label>
    </div>
  </div>
</div>
  `;
  }
}