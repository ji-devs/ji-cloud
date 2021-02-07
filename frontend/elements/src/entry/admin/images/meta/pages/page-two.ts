import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/ji";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/inputs/search";
export type Display = "collapsed" | "expanded";
import { nothing } from "lit-html";
@customElement('image-meta-page-two')
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

    ::slotted([slot=left]){
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

  @property()
  display:Display = "expanded";

  render() {

    const STR_LABEL ="Label Images";
    const STR_CATEGORIES = "Categories";
    const STR_SUMMARY = "Categories Summary"
    const {display} = this;
    const icon = display === "expanded" ? "Icon_CollapseAll_24.svg"
            : display === "collapsed" ? "Icon_ExpandAll_24.svg"
            : nothing;
    return html`

    <div class="main-wrapper">
    <underlined-title  title=${STR_LABEL}></underlined-title>
        <div class="wrapper">
            <slot name="left"></slot>
            <div class="middle">
                <div class="title-wrapper">
                    <title-ji class="title" color="blue">${STR_CATEGORIES}</title-ji>
                    <img-ui path="${icon}"></img-ui>

                </div>
              
                    <slot name="middle"></slot>
                
            </div>
            <div class="right">
            <title-ji color="blue" >${STR_SUMMARY}</title-ji>
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