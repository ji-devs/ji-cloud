import "@elements/icon-wtitle-wparagraph";
export default {
  title: 'Homepage Paragraph',
}


const STR_PATH ="PinkSmiley.jpg";
const STR_TITLE = "content"
const STR_PARAGRAPH = "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more"


export const IconWTitleWParagraph = () => {
    return `
        <icon-wtitle-wparagraph path="${STR_PATH}" title="${STR_TITLE}" paragraph="${STR_PARAGRAPH}"></icon-wtitle-wparagraph>
    `
}

