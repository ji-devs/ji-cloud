import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/inputs/composed/select/select";
import "@elements/core/inputs/composed/select/option";
import "@elements/core/inputs/wrapper";

@customElement("jig-details")
export class JigLabelUI extends LitElement {
  static styles = css`
  #container {
    padding: 10px;
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
      <slot></slot>
    </div>
    `
  }
}