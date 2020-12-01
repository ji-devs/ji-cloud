import {renderTemplate as tmpl} from "@utils/template";
import {setIframeContentsId, setIframeContents} from "@utils/dom";
import { modulePage, ModulePageKind } from "@components/module";
import { makeMainPairs } from "./_utils/main";
import headerEmpty from "@templates/module/memory/edit/_common/header/empty.html";
import footerDefault from "@templates/module/memory/edit/_common/footer/default.html";
import iframeTmpl from "@templates/module/memory/edit/_common/main/iframe.html";

export default {
  title: 'Modules/Memory/Edit/All Steps/Step 4',
}

export const Preview = () => makeStep({});

interface Options {
}


function makeStep({}:Options) {
  const pairKind = "text-image";

  const main = tmpl(iframeTmpl);
  
  setIframeContents(main, "<h1>Player here!</h1>");

  const header = tmpl(headerEmpty);
  const footer = tmpl(footerDefault);

  return modulePage({
    kind: ModulePageKind.EditPlain,
    header,
    main,
    footer,
  })
}