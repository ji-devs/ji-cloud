import "@elements/buttons/rectangle-button";
export default {
  title: 'Rectangle Button',
}

interface ButtonArgs {
  label: string,
  color: string,
  size: string,
  bold: boolean,
  italic: boolean,
  path:string,
  imglefthidden:boolean,
  imgrighthidden:boolean

}

const DEFAULT_ARGS:ButtonArgs = {
  label: "Submit",
  color: "red",
  size: "medium",
  bold: false,
  italic: false,
  path:"",
  imglefthidden:false,
  imgrighthidden:false
}

export const RectangleButton = (props?:ButtonArgs) => {

  const {label, color, size, bold, italic, path, imglefthidden, imgrighthidden} = props || DEFAULT_ARGS;
    return `<rectangle-button label="${label}" color="${color}" size="${size}" ${bold && "bold"} ${imglefthidden && "imglefthidden"} ${imgrighthidden && "imgrighthidden"} ${italic && 'italic' } path="${path}"/>`
}




RectangleButton.args = DEFAULT_ARGS;
