import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {MEDIA_UI} from "@utils/path";
import {mockWords, mockThemes} from "../../common/mock-data";
import step1Page from "@templates/module/memory/edit/words-and-images/step-1/step-1.html";
import step1ImagesRecent from "@templates/module/memory/edit/words-and-images/step-1/sidebar/images-recent.html";
import step1ImageThumbnail from "@templates/module/memory/edit/words-and-images/step-1/sidebar/image-thumbnail.html";

export default {
  title: 'Modules/Memory-Game/Edit/Words And Images/Step1/Sidebar',
}


export const Text = () => {
    let page = tmpl(step1Page);
    toggleClassesId(page, "images-widget", "hidden", true);

    mockWords.forEach(word => {
      appendValueLineId(page, "list-items", word);
    });
    return page;
}

export const Images = () => makeStep1({
  search: false,
  recent: false,
  selectedSearch: false,
});

export const Images_Search = () => makeStep1({
  search: true,
  recent: false,
  selectedSearch: false,
});
export const Images_Search_Selected = () => makeStep1({
  search: true,
  recent: false,
  selectedSearch: true,
});
export const Images_Recent = () => makeStep1({
  search: false,
  recent: true,
  selectedSearch: false,
});
export const Images_Search_And_Recent= () => makeStep1({
  search: true,
  recent: true,
  selectedSearch: false,
});

function makeStep1({recent, search, selectedSearch}) {
    let page = tmpl(step1Page);
    toggleClassesId(page, "text-widget", "hidden", true);

    if(search) {
      appendImagesSearch(page, selectedSearch);
    }
    if(recent) {
      appendImagesRecent(page);
    }
    const sidebar = getChildId(page, "images-widget");

    return page;
}

function appendImagesSearch(page, selected) {
    const sidebar = getChildId(page, "images-widget");
    const search = getChildId(sidebar, "search"); 

    Array(6).fill(0).forEach((_, idx) => {
      const image = tmpl(step1ImageThumbnail, {
        src: `${MEDIA_UI}/sticker-4991-2018-08-17-full.png`
      });

      if(selected && idx == 0) {
        toggleClasses(image, ["row-span-3", "col-span-3"], true);
      } else {
        toggleClasses(image, ["h-83.5", "w-83.5"], true); 
      }
      appendId(search, "list", image);
    });
}

function appendImagesRecent(page) {
    const sidebar = getChildId(page, "images-widget");
    const recent = tmpl(step1ImagesRecent);

    Array(2).fill(0).forEach((_, idx) => {
      const image = tmpl(step1ImageThumbnail, {
        src: `${MEDIA_UI}/sticker-4991-2018-08-17-full.png`
      });

      toggleClasses(image, ["h-83.5", "w-83.5"], true); 
      if(idx == 0) {
        toggleClasses(image, ["jig-image-selected", "hover:h-312", "hover:w-312"], true);
      }

      appendId(recent, "list", image);
    });

    appendId(sidebar, "recent", recent);
}
    /*
    appendMockEditCards(_page, {flipSecond: false, textInput: true});
    setTextId(page, "list-items", "");

    mockWords.forEach(word => {
      appendValueLineId(page, "list-items", word);
    });
    const {id} = mockThemes[0];
    toggleClassesId(page, "cards", [`memory-theme-${id}`], true);
    */
/* To migrate

import memoryimagesone from "@templates/module/memory/edit/images/memory-images-one.html";
import memoryimagestwo from "@templates/module/memory/edit/images/memory-images-two.html";
import addedimage from "@templates/module/memory/edit/images/added-image-tooltip.html";



export default {
  title: 'Modules/Memory-Game/Edit/Images',
}

export const MemoryImagesStepOne = () =>
    tmpl(memoryimagesone, {

});

export const MemoryImagesStepTwo = () =>
    tmpl(memoryimagestwo, {

});

export const AddedImageTooltip = () =>  {
    const pageContainer = tmpl(memoryimagestwo);

    const pageContents = tmpl(addedimage);

    appendId(pageContainer, "memory-added-image", pageContents);

    return pageContainer;
}
*/