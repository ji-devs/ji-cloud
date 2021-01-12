import "@elements/admin/templates-layout/publish-full-two";
import "@elements/image-thumbnail";
import "@elements/placeholder";
import "@elements/inputs/textarea-text";
import "@elements/dividers/spacer-fourty";
import "@elements/titles/plain-blue";
import "@elements/titles/title-w-icon";
import { RectangleButton } from "~/components/rectangle-button";
import { colorStyles } from "@elements/_styles/colors";

export default {
  title: 'Full Pages/Publish',
}

  
  interface PublishArgs {
  
    }

    const DEFAULT_ARGS:PublishArgs = {
      
      }

      const STR_TITLE = "Settings and JIG info.";
      const STR_SUBTITLE = "Last step before publishing";
      const STR_MEDIUM = "medium";
      const STR_WHITE = "white";
      const STR_SHARE = "share the JIG";
      const STR_CREATE = "create a new JIG";
      const STR_PLAY = "play the JIG";
      const STR_ICNSHARE = "Icn_Share_Red.svg";
      const STR_ICNCREATE = "Icn_Plus_Red.svg";
      const STR_ICNPLAY = "icn-video-activity-hover.svg"

      



export const PublishFullTwo = (props?:PublishArgs) => {

 const {} = props || DEFAULT_ARGS;
 

    return `
    <publish-full-two title="${STR_TITLE}" subtitle="${STR_SUBTITLE}">
        <placeholder-img slot="animation"></placeholder-img>
        <div slot="button-collection">${RectangleButton({label:STR_SHARE, imgrighthidden:true, size:STR_MEDIUM,color:STR_WHITE, path:STR_ICNSHARE, imglefthidden:false, bold:false, italic:false })}</div>
        <div slot="button-collection">${RectangleButton({label:STR_CREATE, imgrighthidden:true,size:STR_MEDIUM,color:STR_WHITE, path:STR_ICNCREATE, imglefthidden:false, bold:false, italic:false  })}</div>
        <div slot="button-collection">${RectangleButton({label:STR_PLAY, imgrighthidden:true,size:STR_MEDIUM,color:STR_WHITE, path:STR_ICNPLAY, imglefthidden:false, bold:false, italic:false  })}</div>

        </publish-full-two>
    
    `
}

PublishFullTwo.args = DEFAULT_ARGS;
