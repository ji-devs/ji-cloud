import "@elements/images/ji";
import "@elements/images/ui";
import {mockUiPath} from "~/mock/images";

export default {
  title: 'Images',
}

export const UiImage = ({path}) => {

    return `<img-ui path="${path}"></img-ui>`
}

UiImage.args = {path: mockUiPath}
export const JiImage = ({lib, size, id}) => {
    console.log(process.env);

    return `<img-ji lib=${lib} size=${size} id=${id}></img-ji>`
}

JiImage.args = {
  lib: "global",
  size: "full",
  id: "07ea0cdc-3003-11eb-b60e-9be7e660cd6c"
}


JiImage.argTypes = {
  lib: {
    control: {
      type: 'inline-radio',
      options: ["global", "web", "user"]
    }
  },
  size: {
    control: {
      type: 'inline-radio',
      options: ["original", "full", "thumb"]
    }
  }
}


JiImage.parameters = {
  docs: {
    description: {
      component: "needs to be kept in sync with the Rust side"
    },
  },
}