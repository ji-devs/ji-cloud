import {renderTemplate as tmpl} from "@utils/template";
import components from "@templates/_demo/components.html";

export default {
  title: 'Common/Components',
}

export const SingleItem = () => {
    const page = tmpl(components, {});
    return page;
}