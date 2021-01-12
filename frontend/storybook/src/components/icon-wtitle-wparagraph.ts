import "@elements/icon-wtitle-wparagraph";
export default {
  title: 'Homepage Paragraph',
}


const STR_TITLE ="Rivka";
const STR_TEXT = "Naomi"

export const IconWTitleWParagraph = () => {
    return `
        <icon-wtitle-wparagraph label="${STR_TITLE}" text="${STR_TEXT}"></icon-wtitle-wparagraph>
    `
}

