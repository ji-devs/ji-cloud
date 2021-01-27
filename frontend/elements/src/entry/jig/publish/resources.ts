import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/variants/title-w-icon";
@customElement('resources-column')
export class _ extends LitElement {
  static get styles() {
    return [css`
    title-w-icon{
      cursor:pointer;
    }
    
    `];
  }


  @property({type:Boolean})
  uploaded: boolean = false;
  

  render() {

    const {uploaded} = this;
    
    return html`
    <div>
        <title-w-icon title="Add lesson plan" path="Icn_Add.svg"  .uploaded=${uploaded}></title-w-icon>
        <title-w-icon title="Add Curriculum" path="Icn_Add.svg" .uploaded=${uploaded}></title-w-icon>
        <title-w-icon title="Add activity ideas" path="Icn_Add.svg" .uploaded=${uploaded}></title-w-icon>
        <title-w-icon title="Add link" path="Icn_Add.svg" .uploaded=${uploaded}></title-w-icon>
    </div>
  `;
  }
}