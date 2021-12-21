import { LitElement, html, customElement, property } from "lit-element";

@customElement("jig-details")
export class JigLabelUI extends LitElement {
  @property({type: String}) name!: string;
  @property({type: String}) authorName!: string;
  @property({type: String}) language!: string;
  @property({type: String}) suitableForAge!: string;
  @property({type: String}) description!: string;
  @property({type: String}) keywords!: string;
  
  render() {
    return html`
      <div>
        <label for="name">JIG's name</label>
        <input id="name" type="text" value=${this.name}>
      </div>
      <div>
        <label for="author">Author name</label>
        <input id="author" type="text" value=${this.authorName}>
      </div>
      <div>
        <label for="language">Instruction Language</label>
        <input id="language" type="text" value=${this.language}>
      </div>
      <div>
        <label for="age">Suitable for age</label>
        <input id="age" type="text" value=${this.suitableForAge}>
      </div>
      <div>
        <label for="affiliation">Affiliation</label>
        <input id="affiliation" type="text" value=${this.name}>
      </div>
      <div>
        <label for="description">JIG teacher's description</label>
        <textarea id="description" rows="6" value=${this.description}></textarea>
      </div>
      <div>
        <label for="keywords">Additional keywords</label>
        <textarea id="keywords" rows="6" value=${this.keywords}></textarea>
      </div>
    `
  }
}