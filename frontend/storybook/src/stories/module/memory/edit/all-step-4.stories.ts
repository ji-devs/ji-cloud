import {renderTemplate as tmpl} from "@utils/template";
import { modulePage, ModulePageKind } from "@components/module";
import { makeMainPairs } from "./_utils/main";
import headerEmpty from "@templates/module/memory/edit/_common/header/empty.html";
import footerDefault from "@templates/module/memory/edit/_common/footer/default.html";

export default {
  title: 'Modules/Memory/Edit/All Steps/Step 4',
}

export const Preview = () => makeStep({});

interface Options {
}


function makeStep({}:Options) {
  const pairKind = "text-image";

  const main = makeMainPairs({pairKind, flipSecond: false, isEdit: false, themeIndex: 1}); 
  const header = tmpl(headerEmpty);
  const footer = tmpl(footerDefault);

  return modulePage({
    kind: ModulePageKind.EditPlain,
    header,
    main,
    footer,
  })
}