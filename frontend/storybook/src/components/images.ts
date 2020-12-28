import "@elements/images/ji";
import "@elements/images/ui";
import {MediaLibOptions, MediaSizeOptions} from "@utils/path";
import {injectSlotStr} from "@utils/slot";

export default {
  title: 'Images',
}

/*** Ui ****/
interface UiArgs {
  path: string,
  slot?: string,
}

const DEFAULT_UI_ARGS:UiArgs = {
  path: `sticker-4991-2018-08-17-full.png`
}

export const UiImage = (props?:UiArgs) => {

    const {path, slotStr} = injectSlotStr(props || DEFAULT_UI_ARGS);
    return `<img-ui path="${path}" ${slotStr}></img-ui>`
}

UiImage.args = DEFAULT_UI_ARGS;

/*** Ji - full ****/
interface JiArgs {
  lib: MediaLibOptions,
  size: MediaSizeOptions,
  id: string,  
  slot?: string,
}

const DEFAULT_JI_ARGS:JiArgs = {
  lib: "global",
  size: "full",
  id: "07ea0cdc-3003-11eb-b60e-9be7e660cd6c"
}

export const JiImage = (props?: JiArgs) => {
    const {lib, size, id, slotStr} = injectSlotStr(props || DEFAULT_JI_ARGS);
    return `<img-ji lib="${lib}" size="${size}" id="${id}" ${slotStr}></img-ji>`
}

JiImage.args = DEFAULT_JI_ARGS; 

JiImage.argTypes = {
  lib: {
    control: {
      type: 'inline-radio',
      options: ["global", "web", "user", "mock"]
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

/*** Ji - mock ****/

interface MockJiArgs {
  size: MediaSizeOptions,
  slot?: string,
}

const DEFAULT_JI_MOCK_ARGS:MockJiArgs = {
  size: "thumb"
}

export const MockJiImage = (props?:MockJiArgs) => {
  const {size, slot, slotStr} = injectSlotStr(props || DEFAULT_JI_MOCK_ARGS);

  return `<img-ji lib="mock" size="${size}" id="image.png" ${slotStr}></img-ji>`
}

MockJiImage.argTypes = {
  size: {
    control: {
      type: 'inline-radio',
      options: ["original", "full", "thumb"]
    }
  }
}


MockJiImage.args = DEFAULT_JI_MOCK_ARGS; 