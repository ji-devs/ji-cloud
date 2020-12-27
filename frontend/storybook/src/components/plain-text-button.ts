import "@elements/buttons/plain-text-button";
import "@elements/buttons/replace-delete";
export default {
  title: 'Plain Text Button',
}

export const PlainTextButton = ({label, color, size, bold, italic}) => {
    return `<plain-button label="${label}" color="${color}" size="${size}" ${bold && "bold"} ${italic && 'italic'}/>`
}

export const ReplaceDelete = ({label, color, size, bold, italic}) => {
    return `<replace-delete label="${label}" color="${color}" size="${size}" ${bold && "bold"} ${italic && 'italic'}/>`
}




PlainTextButton.args = {
  label: "Replace",
  color: 'blue',
  size: 'small',
  bold: false,
  italic: false,
}


ReplaceDelete.args = {
    label: "Delete",
    color: 'blue',
    size: 'small',
    bold: false,
    italic: false,
  }
  