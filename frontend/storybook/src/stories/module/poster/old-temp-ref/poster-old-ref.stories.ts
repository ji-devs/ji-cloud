import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import jignavigation from "@templates/module/poster/old-temp-ref/jig-navigation.html";
import jigcoverone from "@templates/module/poster/old-temp-ref/jig-cover1.html";
import jigaddimage from "@templates/module/poster/old-temp-ref/cover-add-image.html";
import jigaddtext from "@templates/module/poster/old-temp-ref/add-text.html";
import jigrecord from "@templates/module/poster/old-temp-ref/cover-record.html";
import jigaudio from "@templates/module/poster/old-temp-ref/cover-audio.html";
import colorpicker from "@templates/module/poster/old-temp-ref/cover-colorpicker.html";
import addcolor from "@templates/module/poster/old-temp-ref/cover-addcolor.html";
import jigaddbackground from "@templates/module/poster/old-temp-ref/cover-addbackground.html";


export default {
  title: 'Modules/Poster/Old Temp Ref',
}

export const JigNavigation = () =>
    tmpl(jignavigation, {

});

export const JigRecord = () =>
    tmpl(jigrecord, {

});

export const JigAudio = () =>
    tmpl(jigaudio, {

});

export const JigCoverOne = () =>
    tmpl(jigcoverone, {

});

export const JigAddImage = () =>
    tmpl(jigaddimage, {

});

export const JigAddText = () =>
    tmpl(jigaddtext, {

});

export const JigAddBackground = () =>
    tmpl(jigaddbackground, {

});

export const JigColorpicker = () =>  {
    const pageContainer = tmpl(jigaddtext);

    const pageContents = tmpl(colorpicker);

    appendId(pageContainer, "colorpicker", pageContents);

    return pageContainer;
}

export const JigAddcolor = () =>  {
    const pageContainer = tmpl(jigaddtext);

    const pageContents = tmpl(addcolor);

    appendId(pageContainer, "addcolor", pageContents);

    return pageContainer;
}
