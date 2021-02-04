import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/variants/horizontal-underlined-title";
@customElement('image-search')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
        padding:30px;
    }
    .results-wrapper{
        margin-top:24px;
        display:flex;
        justify-content:space-between;
    }
    .results{
        display:flex;
    }
    .list-object li{
        padding-left:16px;
    }

    .list-object li::before{
        content:'';
        height:16px;
        width:16px;
        border-radius:50%;
        display:inline-block;
        margin-right:8px;
    }
    .published::before{
        background-color: #6eca90;
    }
    .saved::before{
       background-color: #e36486;

    }
    .images{
        display:grid;
        grid-template-columns:repeat(5, 1fr);
        grid-column-gap: 62px;
        grid-row-gap:40px;
    }
    
 
    `];
  }

  render() {
    const STR_LABEL = "Label Image";
    const STR_FOUND = "We found";
    const STR_IMAGES = "images for";
    const STR_PUBLISHED ="Show published";
    const STR_SAVED = "Show saved";

    return html`
    <main>
        <horizontal-underlined-title  title=${STR_LABEL}>
            <input-search></input-search>
        </horizontal-underlined-title>
        <div class="results-wrapper">
            <div class="results">
                <title-ji color="black" size="medium-large">${STR_FOUND}&nbsp;</title-ji>
                <slot name="number"></slot>
                <title-ji color="black" size="medium-large">${STR_IMAGES}&nbsp;</title-ji>
                <slot name="searchword"></slot>

            </div>
            <slot name="pagination"></slot>
            <div class="dropdown-wrapper">
                <dropdown-underlined>
                <div class="list-object">
                <li class="published">${STR_PUBLISHED}</li>
                </div>
                <div class="list-object">
                <li class="saved">${STR_SAVED}</li>
                </div>
                </dropdown-underlined>
            </div>
        </div>
        <div class="images">
        <slot name="image-display"></slot>
        </div>
    </main>
  `;
  }
}