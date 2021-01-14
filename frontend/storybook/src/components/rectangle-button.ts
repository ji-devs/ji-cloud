import "@elements/buttons/rectangle-button";
import {Color} from "@elements/buttons/rectangle-button";
import {Size} from "@elements/buttons/rectangle-button";
export default {
  title: 'Rectangle Button',
}

interface ButtonArgs {
  label: string,
  color: Color,
  size: Size,
  bold: boolean,
  italic: boolean,
  iconpath:string,
  imglefthidden:boolean,
  imgrighthidden:boolean

}

const DEFAULT_ARGS:ButtonArgs = {
  label: "Submit",
  color: "red",
  size: "medium",
  bold: false,
  italic: false,
  iconpath:"",
  imglefthidden:true,
  imgrighthidden:true
}

export const RectangleButton = (props?:ButtonArgs) => {

  const {label, color, size, bold, italic, iconpath, imglefthidden, imgrighthidden} = props || DEFAULT_ARGS;
    return `<rectangle-button label="${label}" color="${color}" size="${size}" ${bold && "bold"} ${imglefthidden && "imglefthidden"} ${imgrighthidden && "imgrighthidden"} ${italic && 'italic' } iconpath="${iconpath}"/>`
}



//Continuing the previous example
RectangleButton.argTypes = {
  color: {
    control: {
      type: 'inline-radio',
      options: ["red", "blue", "green", "white"]
    }
  },
  size: {
    control: {
      type: 'inline-radio',
      options: ["small", "medium", "large"]
    }
  }
}


RectangleButton.args = DEFAULT_ARGS;
