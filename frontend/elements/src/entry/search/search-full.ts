import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/buttons/rectangle";

@customElement('search-full')
export class _ extends LitElement {
  static get styles() {
    return [css`
        .search-result{
            display:flex;
            margin-top:32px;
            align-items:center;
            justify-content:center;
        }
        .search-result title-ji{
            margin-right:6px;
            margin-left:6px;
        }
        .jig-section{
          width:1656px;
          margin-left:auto;
          margin-right:auto;
          margin-bottom: 88px;
      }
      .section{
          display:grid;
          grid-template-columns:repeat(4, 1fr);
          row-gap:80px;
          column-gap:80px;
      }
      .title-wrapper{
        display:flex;
        align-items:center;
        position:relative;
        margin-bottom:48px;
        justify-content:space-between;
      }
      .jigimage{
        position:absolute;
        top:0;
        left:-50px;
      }
      .inside-title-wrapper{
        display:flex;
        align-items:center;
      }
      button-rect{
        margin-top: 86px;
        display:flex;
        justify-content:center;
      }
      .learningimage{
        position:absolute;
        top: -30px;
        left: -108px;
      }
      .inside-title-wrapper title-ji:first-of-type{
        margin-right:8px;
      }

      
    
        
    `];
  }


  @property ()
  jignumber:string = "";

  @property ()
  learningnumber:string = "";

  @property ()
  recommendednumber:string = "";

  render() {

    const {jignumber,learningnumber,recommendednumber } = this;
    const STR_FOUND = "We found";
    const STR_RESULTS = "results for";
    const STR_JIG ="JIGs";
    const STR_SORTING ="Sort by Highest rating";
    const STR_LEARNING = "Learning Paths";
    const STR_RECOMMENDED = "Recommended for you";
    
    return html`    
    <main>
      <slot name="header"></slot>
      <div class="search-result">
        <title-ji color="black" size="title-large">${STR_FOUND}</title-ji>
        <slot name="results"></slot>
        <title-ji color="black" size="title-large">${STR_RESULTS}</title-ji>
        <slot name="phrase"></slot>
      </div>
      <div class="jig-section">
        <div class="title-wrapper">
          <div class="inside-title-wrapper">
            <title-ji size="title-large" weight="bolder" color="darkblue">${STR_JIG}</title-ji>
            <title-ji size="title-medium" weight="bolder" color="darkblue">${learningnumber}</title-ji>
            <img-ui class="jigimage" path="jig-cover.svg"></img-ui>
          </div>
          <div>
            <title-ji color="black" size="medium" weight="normal">${STR_SORTING}</title-ji>
           
          </div>
          
        </div>

        <div class="section">
          <slot name="card"></slot>
          
        </div>
        <button-rect color="blue" size="medium">See more</button-rect>
    
      </div>
      <div class="jig-section">
        <div class="title-wrapper">
          <div class="inside-title-wrapper">
            <title-ji size="title-large" weight="bolder" color="darkblue">${STR_LEARNING}</title-ji>
            <title-ji size="title-medium" weight="bolder" color="darkblue">${jignumber}</title-ji>
            <img-ui class="learningimage" path="group-14513.svg"></img-ui>
          </div>
          <div>
            <title-ji color="black" size="medium" weight="normal">${STR_SORTING}</title-ji>
            
          </div>
          
        </div>

        <div class="section">
          <slot name="learncard"></slot>
        </div>
        <button-rect color="blue" size="medium">See more</button-rect>

        
    
      </div>
      <div class="jig-section">
        <div class="title-wrapper">
          <div class="inside-title-wrapper">
            <title-ji size="title-large" weight="bolder" color="darkblue">${STR_RECOMMENDED}</title-ji>
            <title-ji size="title-medium" weight="bolder" color="darkblue">${recommendednumber}</title-ji>
            <img-ui class="jigimage" path="jig-cover.svg"></img-ui>
          </div>
          <div>
            <title-ji color="black" size="medium" weight="normal">${STR_SORTING}</title-ji>
           
          </div>
          
        </div>

        <div class="section">
          <slot name="recommendedcard"></slot>
          
        </div>
        <button-rect color="blue" size="medium">See more</button-rect>
    
      </div>
      
    <slot name="footer"></slot>
    </main>

  `;
  }
}