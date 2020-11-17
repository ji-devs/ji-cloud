import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
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
