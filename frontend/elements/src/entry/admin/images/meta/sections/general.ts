import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/ji";
import "@elements/core/inputs/composed/search";
import "@elements/core/lists/list-vertical";
import { nothing } from "lit-html";

const STR_STYLE = "Image style";
const STR_AGE = "Suitable for age";
const STR_AFFILIATION = "Suitable for jewish stream?";
const STR_TAGS = "Tags (internal use only)";

@customElement('image-meta-section-general')
export class _ extends LitElement {
  static get styles() {
    return [css`
        .container1 {
            display: flex;
        }
        .container2 {
            display: flex;
            flex-direction: column;
        }
    `];
  }

  render() {

    return html`
        <section>
            <div class="container1">
                <list-vertical label="${STR_STYLE}">
                    <slot name="styles"></slot>
                </list-vertical>
                <list-vertical label="${STR_TAGS}">
                    <slot name="tags"></slot>
                </list-vertical>
                <div class="container2">
                    <list-vertical label="${STR_AGE}">
                        <slot name="age_ranges"></slot>
                    </list-vertical>
                    <list-vertical label="${STR_AFFILIATION}">
                        <slot name="affiliations"></slot>
                    </list-vertical>
                </div>
            </div>
        </section>
  `;
  }
}
