import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/ji";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/inputs/search";
@customElement('image-meta-page-three')
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
    align-items:center;
    justify-content:space-between
}
.summary-title{
    display:block;
    margin-top:24px;
    
}

.summary-title:first-of-type{
    margin-top:0;
}
   



   
    `];
    }

    render() {

        const { } = this;
        const STR_LABEL = "Label Images";
        const STR_GENERAL = "General Summary";
        const STR_EDIT = "Edit";
        const STR_IMAGENAME = "Image name";
        const STR_DESCRIPTIONTITLE = "Image description";
        const STR_STYLETITLE = "Image style";
        const STR_USED = "To be used only for";
        const STR_AGE = "Suitable for age";
        const STR_STREAM = "Suitable for jewish stream";
        const STR_CATEGORIES = "Categories Summary";

        return html`
    <div class="main-wrapper">
        <underlined-title title="${STR_LABEL}"></underlined-title>
        <div class="wrapper">
            <slot name="left"></slot>
            <div class="middle">
                <div class="title-wrapper">
                    <title-ji class="title" color="blue">${STR_GENERAL}</title-ji>
                    <title-ji class="title" color="blue">${STR_EDIT}</title-ji>
    
                </div>
                <card-blue class="slot-wrapper">
                    <title-ji color="blue" class="summary-title">${STR_IMAGENAME}</title-ji>
                    <slot name="imagename"></slot>
                    <title-ji color="blue" class="summary-title">${STR_DESCRIPTIONTITLE}</title-ji>
                    <slot name="description"></slot>
                    <title-ji color="blue" class="summary-title">${STR_STYLETITLE}</title-ji>
                    <slot name="style"></slot>
                    <title-ji color="blue" class="summary-title">${STR_USED}</title-ji>
                    <slot name="used"></slot>
                    <title-ji color="blue" class="summary-title">${STR_AGE}</title-ji>
                    <slot name="age"></slot>
                    <title-ji color="blue" class="summary-title">${STR_STREAM}</title-ji>
                    <slot name="stream"></slot>
                </card-blue>
    
            </div>
            <div class="right">
    
                <div class="title-wrapper">
                    <title-ji color="blue">${STR_CATEGORIES}</title-ji>
                    <title-ji class="title" color="blue">${STR_EDIT}</title-ji>
    
                </div>
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