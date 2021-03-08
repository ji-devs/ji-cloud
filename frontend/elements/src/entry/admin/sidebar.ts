import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

export type SECTION = "" | "locale" | "image-add" | "image-search" | "jig" | "category";

const STR_IMAGE_ADD = "Add image";
const STR_IMAGE_SEARCH = "Edit images";
const STR_JIG = "Label JIGs";
const STR_CATEGORIES = "Edit categories";
const STR_LOCALE = "Localization";

@customElement('admin-sidebar')
export class _ extends LitElement {
  static get styles() {
    return [css`
    section.open {
        width: 259px;
        height:100%;
        background-color: #83aef7;
        position:relative;
        min-height:100vh;
    

    }

    section.closed{
        width: 32px;
        display: flex;
        justify-content: center;
        height:100%;
        background-color: #83aef7;
        position:relative;
        min-height:100vh;
    }

    .open, .close {
        cursor: pointer;
    }

    .close {
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
    .list-option.selected, .list-option:hover{
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

  onRoute(route: SECTION) {
      const {categoryLocked, imageLocked, jigLocked} = this;
      if((route === "category" && categoryLocked) || (route === "jig" && jigLocked) || ((route === "image-add" || route === "image-search") && imageLocked)) {
          return;
      }
      this.dispatchEvent(
          new CustomEvent("custom-route", {
              detail: { route},
              composed: true,
              bubbles: true
          })
      );
  }


  @property()
  section:SECTION = ""; 

  @property()
  label:string = ""; 

  @property({type: Boolean})
  categoryLocked:boolean = false;

  @property({type: Boolean})
  imageLocked:boolean = false;

  @property({type: Boolean})
  jigLocked:boolean = false;

  @property({type: Boolean})
  localeLocked:boolean = false;

  @property({type: Boolean})
  closed:boolean = false;

  render() {
    const {categoryLocked, imageLocked, jigLocked, localeLocked, closed, section} = this;

    return closed 
        ? html`
            <section class="closed">
                <div class="open" @click=${() => this.closed = !this.closed}>>></div>
            </section>
        `
        : html`
            <section class="open">
                <img-ui path="core/buttons/icon/x.svg" class="close" @click=${() => this.closed = !this.closed}></img-ui>
            <img-ui path="entry/admin/sidebar/logo-ji-blue.svg" class="logo"></img-ui>
            <div class=${classMap({["list-option"]: true, selected: section === "image-add"})} @click=${() => this.onRoute("image-add")}>
                <p>${STR_IMAGE_ADD}</p>
                ${imageLocked ? html`<img-ui path="entry/admin/sidebar/lock.svg"></img-ui>` : nothing}
            </div>
            <div class=${classMap({["list-option"]: true, selected: section === "image-search"})} @click=${() => this.onRoute("image-search")}>
                <p>${STR_IMAGE_SEARCH}</p>
                ${imageLocked ? html`<img-ui path="entry/admin/sidebar/lock.svg"></img-ui>` : nothing}
            </div>
            <div class=${classMap({["list-option"]: true, selected: section === "jig"})} @click=${() => this.onRoute("jig")}>
                <p>${STR_JIG}</p>
                ${jigLocked ? html`<img-ui path="entry/admin/sidebar/lock.svg"></img-ui>` : nothing}
            </div>
            <div class=${classMap({["list-option"]: true, selected: section === "category"})} @click=${() => this.onRoute("category")}>
                <p>${STR_CATEGORIES}</p>
                ${categoryLocked ? html`<img-ui path="entry/admin/sidebar/lock.svg"></img-ui>` : nothing}
            </div>
            <div class=${classMap({["list-option"]: true, selected: section === "locale"})} @click=${() => this.onRoute("locale")}>
                <p>${STR_LOCALE}</p>
                ${localeLocked ? html`<img-ui path="entry/admin/sidebar/lock.svg"></img-ui>` : nothing}
            </div>
            </section>
        `;
  }
}
