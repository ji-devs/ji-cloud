import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/variants/title-section";
import "@elements/core/buttons/text";
import "@elements/entry/home/TOSORT/create-leftparagraph";

@customElement('jiggling-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;

    }
   main{
     width:1920px;
     height:890px;
     background-image: url("path/to/Strip_Background_Jigglings@2x.jpg");
      background-repeat: no-repeat;
     background-attachment: fixed;
     background-position: center;
 

   }
   
   .title{
    font-size: 64px;
    font-weight: 900;
    color:#5662a3;
    text-align: center;
    display:block;
    margin-top:62px;

   }
   .icon-wtitle-wparagraph{
     margin-left:25px;
     display:block;
   }

 
    `];
  }



  render() {

    const { } = this;

   
    const STR_TITLE_PinkSmiley = "Content";
    const STR_PARAGRAPH_PinkSmiley = "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more";
    const STR_BUTTONLABEL_PinkSmiley = "See our templates";

    
    const STR_TITLE_BlueWheel = "Create";
    const STR_PARAGRAPH_BlueWheel = "Create your own activities, Teach your class to create their own games. The most fun way to learn something new.";
    const STR_BUTTONLABEL_BlueWheel = "Try it for free";

    const STR_TITLE_GreenRectangle = "Customize";
    const STR_PARAGRAPH_GreenRectangle = "Easily, saving time way. Customize our templates for your needs. ";
    const STR_BUTTONLABEL_GreenRectangle = "See our templates";

    const STR_TITLE_YellowSquare = "Community";
    const STR_PARAGRAPH_YellowSquare = "Meet X users around the world. See who plays now. Meet other teachers.";
    const STR_BUTTONLABEL_YellowSquare = "Get inspired";

    const STR_TITLE_BlueTriangle = "Classroom";
    const STR_PARAGRAPH_BlueTriangle = "track your students journey, manage your lessons, See which activities are more successful.";
    const STR_BUTTONLABEL_BlueTriangle = "Manage your class";
    const STR_TITLE = "Why Ji?";



    return html`
    <main>
    <title-section titlecolor="purple" title="${STR_TITLE}" class="title"></title-section>
    <div class="inside-wrapper">
    <icon-wtitle-wparagraph class="icon-wtitle-wparagraph" path="Jiggling_Content@2x.png" title="${STR_TITLE_PinkSmiley}" paragraph="${STR_PARAGRAPH_PinkSmiley}" color="pink" >
    <button-text color="blue" size="small" weight="normal" italic=false >${STR_BUTTONLABEL_PinkSmiley}</button-text>
 </icon-wtitle-wparagraph>

 <icon-wtitle-wparagraph class="icon-wtitle-wparagraph" path="Jiggling_Creator@2x.png" title="${STR_TITLE_BlueWheel}" paragraph="${STR_PARAGRAPH_BlueWheel}" color="darkblue" >
 <button-text color="blue" size="small" weight="normal" italic=false >${STR_BUTTONLABEL_BlueWheel}</button-text>
</icon-wtitle-wparagraph>

<icon-wtitle-wparagraph class="icon-wtitle-wparagraph" path="Jiggling_Customize@2x.png" title="${STR_TITLE_GreenRectangle}" paragraph="${STR_PARAGRAPH_GreenRectangle}" color="green" >
<button-text color="blue" size="small" weight="normal" italic=false >${STR_BUTTONLABEL_GreenRectangle}</button-text>
</icon-wtitle-wparagraph>

<icon-wtitle-wparagraph class="icon-wtitle-wparagraph" path="Jiggling_Community@2x.png" title="${STR_TITLE_YellowSquare}" paragraph="${STR_PARAGRAPH_YellowSquare}" color="orange" >
<button-text color="blue" size="small" weight="normal" italic=false >${STR_BUTTONLABEL_YellowSquare}</button-text>
</icon-wtitle-wparagraph>

<icon-wtitle-wparagraph class="icon-wtitle-wparagraph" path="Jiggling_Classroom@2x.png" title="${STR_TITLE_BlueTriangle}" paragraph="${STR_PARAGRAPH_BlueTriangle}" color="lightblue" >
<button-text color="blue" size="small" weight="normal" italic=false >${STR_BUTTONLABEL_BlueTriangle}</button-text>
</icon-wtitle-wparagraph>

        <slot name="icon-title-paragraph"></slot>
    </div>
    </main>
  `;
  }
}