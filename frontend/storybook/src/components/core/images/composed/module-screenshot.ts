import "@elements/core/images/composed/module-screenshot";
import { ModuleKind } from "@elements/module/_common/types";
import { argsToAttrs } from "@utils/attributes";
import {MediaLibOptions, MediaSizeOptions} from "@utils/path";
import {injectSlotStr} from "@utils/slot";

export default {
  title: 'Core / Images / Composed',
}

interface Args {
  jigId: string,
  moduleId: string,
  fallbackKind: ModuleKind,
  size: MediaSizeOptions,
}

const DEFAULT_ARGS:Args = {
  size: "thumb",
  jigId: "cade13b0-f9f1-11eb-a8bc-e3d516c9c78b",
  moduleId: "cae01408-f9f1-11eb-a8bc-93cb1ce43c8d",
  fallbackKind: "cover"
}

export const ModuleScreenshot = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
  return `<img-module-screenshot ${argsToAttrs(props)}></img-module-screenshot>`;
}


ModuleScreenshot.argTypes = {
  size: {
    control: {
      type: 'inline-radio',
      options: ["original", "full", "thumb"]
    }
  },
}
ModuleScreenshot.args = DEFAULT_ARGS; 
