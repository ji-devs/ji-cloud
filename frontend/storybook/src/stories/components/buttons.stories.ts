import {renderTemplate as tmpl} from "@utils/template";
import {appendId} from "@utils/dom";
import buttons from "@templates/_common/buttons/button-components.html";

export default {
  title: 'Components/Buttons',
}

export const Buttons = () =>
    tmpl(buttons, {});
