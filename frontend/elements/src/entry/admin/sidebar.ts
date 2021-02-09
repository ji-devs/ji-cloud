import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('admin-sidebar')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
        width: 259px;
        height:100%;
        background-color: #83aef7;
        position:relative;
        min-height:100vh;
    

    }
    .close{
        position:absolute;
        top:20px;
        right:12px;
    }
    .logo{
        margin-left:40px;
        margin-top:20px;
        margin-bottom:84px;
        display:inline-block;
    }
    .list-option{
        max-width:259px;
        height:56px;
        cursor:pointer;
        display:flex;
        align-items:center;
        border-left:solid 8px #83aef7;
        justify-content:space-between;
        padding-right:20px;
        
    }
    .list-option:hover{
        background-color: #6698ed;
        border-left:solid 8px #2b54b8;

    }
    p{
        font-size: 18px;
        font-weight: 500;
        margin-left:40px;
        
        
    }
    `];
  }



  @property()
  label:string = ""; 

  render() {
    const STR_IMAGES = "Label images";
    const STR_JIG = "Label JIGs";
    const STR_EDIT = "Edit Categories";

    return html`
    <main>
    <img-ui path="icon-close-menu.svg" class="close"></img-ui>
    <img-ui path="logo-ji-blue.svg" class="logo"></img-ui>
    <div class="list-option">
    <p>${STR_IMAGES}</p>
    </div>
    <div class="list-option">
    <p>${STR_JIG}</p>
    </div>
    <div class="list-option">
    <p>${STR_EDIT}</p>
    <img-ui path="lock-24-px.svg"></img-ui>
    </div>
    </main>

  `;
  }
}