import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('card-dropdown')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
      display:flex;
      overflow:hidden;
      scroll:auto;
       
    }
    .collapsed{
        height:44px;
        overflow:hidden;
    }
    .expanded{
        height:100%;
    }
    .collapsed ::slotted([slot=content]){
        display:none;
    }
    `];
  }

  @property({type:Boolean})
  collapsed: boolean = true;

  @property()
  icon: string = "";

  @property()
  label: string = "";

  render() {

    const {collapsed,icon,label} = this;

    return html`
     <main class="${collapsed ? 'collapsed' : 'expanded'}">
        <div class="content-wrapper">
            <slot name="title">${label}</slot>
            <slot name="content"></slot>
        </div>
        
        <img-ui path="${icon}"></img-ui>
    </main>
  `;
  }
}