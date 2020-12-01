import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, addClasses, appendTextLineId, addClassesId, setTextId} from "@utils/dom";
import { modulePage, ModulePageKind } from "@components/module";
import { makeMainPairs } from "./_utils/main";
import { getAllThemes} from "../_config/themes";
import sidebarTmpl from "@templates/module/memory/edit/_common/sidebar/step2.html";
import itemSelected from "@templates/module/memory/edit/_common/sidebar/step2-theme-item-selected.html";
import itemDeselected from "@templates/module/memory/edit/_common/sidebar/step2-theme-item-deselected.html";
import headerEmpty from "@templates/module/memory/edit/_common/header/empty.html";
import footerDefault from "@templates/module/memory/edit/_common/footer/default.html";

export default {
  title: 'Modules/Memory/Edit/All Steps/Step 2',
}

export const Step2 = () => makeStep({themeIndex: 1, flipSecond: false});
export const Step2_Theme2 = () => makeStep({themeIndex: 2, flipSecond: false});
export const Step2_Theme2_FlipSecond = () => makeStep({themeIndex: 2, flipSecond: true});

interface Options {
  themeIndex: number,
  flipSecond: boolean
}


function makeStep({themeIndex, flipSecond}:Options) {
  const pairKind = "text-image";

  const main = makeMainPairs({pairKind, flipSecond, isEdit: false, themeIndex}); 
  const header = tmpl(headerEmpty);
  const footer = tmpl(footerDefault);

  const sidebar = tmpl(sidebarTmpl);

  getAllThemes() 
    .forEach(({content, label, id}, idx) => {
        const item = idx === themeIndex ? tmpl(itemSelected) : tmpl(itemDeselected);
        const left = getChildId(item, "left");
        setTextId(left, "text-contents", content);

        setTextId(item, "label", label);

        addClasses(item, [`memory-theme-${id}`]);
        
        appendId(sidebar, "theme-items", item);
    });

  return modulePage({
    kind: ModulePageKind.EditPlain,
    sidebar,
    header,
    main,
    footer,
  })
}