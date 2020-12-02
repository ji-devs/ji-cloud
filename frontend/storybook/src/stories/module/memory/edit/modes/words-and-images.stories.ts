
import { renderTemplate as tmpl } from "@utils/template";
import { appendId, appendValueLineId} from "@utils/dom";

import {getThemeByIndex} from "../../_config/themes";
import {getInitialWords} from "../../_config/initial-words";
import { makeMainPairs } from "../_utils/main";
import { modulePage, ModulePageKind } from "@components/module";
import { SearchWidget } from "@components/image-search";

import sidebarStep1Empty from "@templates/module/memory/edit/words-and-images/sidebar/step1-empty.html";
import sidebarStep1Words from "@templates/module/memory/edit/words-and-images/sidebar/step1-words.html";
import sidebarStep1Images from "@templates/module/memory/edit/words-and-images/sidebar/step1-images.html";
import headerEmpty from "@templates/module/memory/edit/_common/header/empty.html";
import headerAddPair from "@templates/module/memory/edit/_common/header/add-pair.html";
import mainEmpty from "@templates/module/memory/edit/_common/main/empty.html";
import footerDefault from "@templates/module/memory/edit/_common/footer/default.html";

export default {
  title: 'Modules/Memory/Edit/Words and Images',
}

export const Step1_Init = () => {
  const main = tmpl(mainEmpty);
  const header = tmpl(headerEmpty);
  const footer = tmpl(footerDefault);
  const sidebar = tmpl(sidebarStep1Words);

  getInitialWords().forEach(word => {
    appendValueLineId(sidebar, "list-items", word);
  });

  return modulePage({
    kind: ModulePageKind.EditPlain,
    sidebar,
    header,
    main,
    footer,
  })
}

export const Step1_Edit_Text = () => {
  const main = makeMainPairs({pairKind: "text-image", flipSecond: false, isEdit: true, themeIndex: 1}); 
  const header = tmpl(headerAddPair);
  const footer = tmpl(footerDefault);
  const sidebar = tmpl(sidebarStep1Empty);

  return modulePage({
    kind: ModulePageKind.EditPlain,
    sidebar,
    header,
    main,
    footer,
  })
}

export const Step1_Edit_Images = () => {
  const main = makeMainPairs({pairKind: "text-image", flipSecond: false, isEdit: true, themeIndex: 1}); 
  const header = tmpl(headerAddPair);
  const footer = tmpl(footerDefault);
  const sidebar = tmpl(sidebarStep1Images);

  const widget = SearchWidget({showRecent: false, showSelectedResult: false});
  appendId(sidebar, "image-search", widget);

  return modulePage({
    kind: ModulePageKind.EditPlain,
    sidebar,
    header,
    main,
    footer,
  })
}