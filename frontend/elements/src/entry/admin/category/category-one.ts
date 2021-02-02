import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/ji";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/inputs/search";
@customElement('category-one')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .main-wrapper{
        padding:40px;
    }
    .wrapper{
        display:flex;
       padding-top:40px;
       border-bottom: solid 1px #e5e7ef;
     
    }

    .left{
      padding-right: 64px;
      border-right:solid 1px #e5e7ef;
      height: 700px;
      
    }
    .middle{
        padding-left:40px;
        margin-right:24px;
    }
    .slot-wrapper{
        display:block;
        margin-top:18px;
  }
  .right{
    width:100%;
}

  ::slotted([slot=button]){
    padding-top: 24px;
    display:flex;
    justify-content: flex-end;
}
::slotted([slot="middle"]){
   display:block;
   margin-top:18px;
}
.title-wrapper{
    display:flex;
    align-items:center
}
.title-wrapper .title{
    margin-right: 20px
}

   
    `];
  }

  render() {

    const STR_LABEL ="Label Images";
    const STR_CATEGORIES = "Categories";
    const STR_SUMMARY = "Categories Summary"
    const STR_BACK = "Back";

    return html`
    <div class="main-wrapper">
    <underlined-title  title=${STR_LABEL}></underlined-title>
        <div class="wrapper">
            <div class="left">
            <slot name="left"></slot>
            <title-ji color="blue">${STR_BACK}</title-ji>
            </div>
            <div class="middle">
                <div class="title-wrapper">
                    <title-ji class="title" color="blue">${STR_CATEGORIES}</title-ji>
                    <input-search></input-search>
                </div>
              
                    <slot name="middle"></slot>
                
            </div>
            <div class="right">
            <title-ji color="blue">${STR_SUMMARY}</title-ji>
            <card-blue class="slot-wrapper">
                <slot name="right"></slot>
                </card-blue>
            </div>

        </div>
        <slot name="button"></slot>
    </div>  
  `;
  }
}