import "@elements/entry/home/TOSORT/column-list";
import "@elements/entry/popup/sections/logout-section";
import "@elements/core/titles/variants/title-section";
 import "@elements/core/buttons/rectangle" ;
import "@elements/core/popups/popup";
export default {
  title: 'Popups',
}

 const STR_TITLE="Logout";
const STR_PATH_PinkSmiley="Jiggling_Content@2x.png";
const STR_TITLE_PinkSmiley = "Content";
const STR_PARAGRAPH_PinkSmiley = "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more";
const STR_BUTTONLABEL_PinkSmiley = "See our templates";
 const STR_SMALL ="small"
 const STR_LINE1="If you are using a public computer, remember to";
 const STR_LINE2=" log out when you’re done.";

 const STR_LINE3="Do you want to logout? ";
const STR_MEDIUM="medium";
const STR_LARGE="large"

const STR_DARKBLUE = "darkblue";
const STR_BLUE = "blue";
const STR_PEACH = "peach";

const STR_TRUE = "true";
const STR_LOGOUT="Logout";

const STR_LABEL="Logout";
const STR_Cancel="Cancel";

export const logoutParagraph = () => {
    return `
<template-popups color="${STR_PEACH}" size="${STR_MEDIUM}">

<logout-section >
     <title-section titlecolor="${STR_DARKBLUE}" title="${STR_TITLE}" size="${STR_SMALL}" slot="title"></title-section>
     <column-list slot="line" text_line="${STR_LINE1}" size="${STR_MEDIUM}"></column-list>
     <column-list slot="line" text_line="${STR_LINE2}" size="${STR_MEDIUM}"></column-list>
     <column-list slot="line" text_line="${STR_LINE3}" size="${STR_MEDIUM}"></column-list>
     <button-rect size="${STR_MEDIUM}" label="${STR_LOGOUT}" color="${STR_BLUE}"     imgrighthidden="${STR_TRUE}" iconpath=""></button-rect>
<button-rect size="${STR_LARGE}" color="${STR_BLUE}" slot="button">${STR_LABEL}</button-rect>
<button-text  size="${STR_LARGE}"  color="${STR_BLUE}" slot="textbutton" >${STR_Cancel}</button-text>
</logout-section>

</template-popups>
    `
}