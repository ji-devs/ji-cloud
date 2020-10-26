import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import jignavigation from "@templates/jig/design/jig-navigation.html";
import jigcoverone from "@templates/jig/design/jig-cover1.html";
import jigaddimage from "@templates/jig/design/cover-add-image.html";
import jigaddtext from "@templates/jig/design/add-text.html";
import jigrecord from "@templates/jig/design/cover-record.html";
import jigaudio from "@templates/jig/design/cover-audio.html";
import colorpicker from "@templates/jig/design/cover-colorpicker.html";
import addcolor from "@templates/jig/design/cover-addcolor.html";
import jigaddbackground from "@templates/jig/design/cover-addbackground.html";


export default {
  title: 'JIG/Design',
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
