
import addons from '@storybook/addons';
import { STORY_RENDERED } from '@storybook/core-events';
import {startResizerOnElement} from "../src/utils/resize";
import {getChildId} from "../src/utils/dom";

export const parameters = {
  actions: { argTypesRegex: "^on[A-Z].*" },
}


addons.getChannel().on(STORY_RENDERED, (story) => {
  startResizerOnElement(getChildId(document, "module-outer", true));
});