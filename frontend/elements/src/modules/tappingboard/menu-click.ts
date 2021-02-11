import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('menu-click')
export class _ extends LitElement {
  static get styles() {
    return [css`
main{
    width: 389px;
    height: 387px;
    border-radius: 24px;
    background-color: #b5d3ff;
    display:flex;
    flex-direction:column;
    align-items:center;
}
.content{
    width: 302px;
  height: 170px;
  background-color:#ffffff;
  border-radius: 16px;
  margin-top:32px;

}
p{
    margin-top:64px;
    font-size: 24px;
    font-weight: 300;
}
 
    `];
  }



  render() {

    const {} = this;
    const STR_TAP = "Tap words & hear";
    return html`
    <main>
    <p>${STR_TAP}</p>
    <div class="content"></div>
   
    </main>
  `;
  }
}