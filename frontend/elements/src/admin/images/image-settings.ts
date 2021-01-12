import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/inputs/checkbox";
import "@elements/buttons/replace-delete";
import "@elements/dividers/vertical-full";



const STR_PREMIUM = "Premium Image";

@customElement('image-settings')
export class _ extends LitElement {
  static get styles() {
    return [css`
    ::slotted(*) {
        width: 288px;
        height: 216px;
       
    }

    main{
      width:288px;
    }
    .wrapper{
      display:flex;
      justify-content: flex-end;
      margin-bottom: 16px;
    }
    vertical-full{
      margin-bottom:16px;
      display:block;
    }

    `];
  }

  @property()
  path:string = ""; 

  @property()
  size: string = "";

  @property()
  label: string = "";

  @property()
  color: "red" | "blue" | "" = "";

  @property({type: Boolean})
  bold:boolean = false; 

  @property({type: Boolean})
  italic:boolean = false; 

  render() {

    const {label, color, size, bold, italic} = this;
// The slot is intended for images
    return html`
    <main>
     <slot name="one"></slot>
     <div class="wrapper">
      <replace-delete></replace-delete>
     </div>
     <vertical-full></vertical-full>
    <input-checkbox label="${STR_PREMIUM}"></input-checkbox>
    <slot name="two"></slot>
    </main>
  `;
  }
}