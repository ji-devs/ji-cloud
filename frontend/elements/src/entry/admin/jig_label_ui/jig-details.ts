import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/inputs/composed/select/select";
import "@elements/core/inputs/composed/select/option";
import "@elements/core/inputs/wrapper";
import "@elements/core/buttons/rectangle";

@customElement("jig-details")
export class JigLabelUI extends LitElement {
  static styles = css`
  #container {
    padding: 44px;
  }
  #heading {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  #general-summary {
    height: 100%;
    color: var(--dark-gray-5);
  }
  #heading-buttons {
    display: flex;
    gap: 0 32px;
  }
  #input-container {
    margin-top: 12px;
    padding: 31px 24px;
    border-radius: 12px;
    border: solid 2px #e6f0ff;
  }
  input-wrapper, input-select {
    margin-top: 24px;
  }
  [label="JIG's name"] {
    margin-top: 0;
  }
  `;
  
  @property({type: String}) name: string = ""
  @property({type: String}) authorName: string = ""
  @property({type: String}) language: string = ""
  @property({type: String}) suitableForAge: string = ""
  @property({type: String}) description: string = ""
  @property({type: String}) keywords: string = ""
  
  render() {
    return html`
    <div id="container">
      <div id="heading">
        <div>
          <div id="general-summary">General Summary</div>
        </div>
        <div id="heading-buttons">
          <button-rect kind="text" color="blue">Cancel</button-rect>
          <button-rect kind="outline" color="blue">Save Changes</button-rect>
        </div>
      </div>
      <div id="input-container">
        <input-wrapper label="JIG's name">
          <input type="text" value=${this.name}>
        </input-wrapper>
        <input-wrapper label="Author name">
          <input type="text" value=${this.authorName}>
        </input-wrapper>
        <input-select label="Instruction Language">
          <input-select-option>English</input-select-option>
          <input-select-option>Spanish</input-select-option>
          <input-select-option>Hebrew</input-select-option>
          <input-select-option>French</input-select-option>
          <input-select-option>Italian</input-select-option>
        </input-select>
        <input-select label="Suitable for age">
          <input-select-option>All ages</input-select-option>
          <input-select-option>No ages</input-select-option>
        </input-select>
        <input-select label="Affiliation">
          <input-select-option>Affiliation 1</input-select-option>
          <input-select-option>Affiliation 2</input-select-option>
          <input-select-option>Affiliation 3</input-select-option>
        </input-select>
        <input-wrapper label="JIG teacher's description">
          <textarea id="description" rows="6" value=${this.description}></textarea>
        </input-wrapper>
        <input-wrapper label="Additional keywords">
          <textarea rows="6" value=${this.keywords}></textarea>
        </input-wrapper>
      </div>
    </div>
    `
  }
}