import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";


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
    `];
  }

  @property({type: Boolean})
  closed:boolean = false;

  render() {
    const {closed} = this;

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
                <slot></slot>
            </section>
        `;
  }
}
