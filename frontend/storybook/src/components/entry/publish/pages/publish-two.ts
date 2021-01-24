import "@elements/entry/jig/publish/page-2";
import "@elements/entry/home/TOSORT/image-thumbnail"; 
import "@elements/entry/home/TOSORT/placeholder"; 
import "@elements/widgets/tags/icon";
import "@elements/entry/jig/publish/publish-dropdown";
import "@elements/core/inputs/textarea";
import "@elements/core/dividers/spacer-fourty";
import "@elements/core/titles/variants/title-w-icon";
import { Rectangle } from "~/components/core/buttons/rectangle";
import { colorStyles } from "@elements/_styles/colors";

export default {
  title: 'Entry/Jig/Publish/Pages',
}

  
  interface PublishArgs {
    closed:boolean,
  
    }

    const DEFAULT_ARGS:PublishArgs = {
       closed:true,
      }

      const STR_TITLE = "Your JIG is on air now";
      const STR_SUBTITLE = "What would you like to do next?";
      const STR_MEDIUM = "medium";
      const STR_WHITE = "white";
      const STR_SHARE = "share the JIG";
      const STR_CREATE = "create a new JIG";
      const STR_PLAY = "play the JIG";
      const STR_ICNSHARE = "Icn_Share_Red.svg";
      const STR_ICNCREATE = "Icn_Plus_Red.svg";
      const STR_ICNPLAY = "icn-video-activity-hover.svg";
      const STR_DROPDOWNTITLE = "Select Share Option";
      const STR_STUDENT = "icn-student.svg";
      const STR_SHARESTUDENT ="Share with Students";
      const STR_URLICON = "icn-url.svg";
      const STR_URL ="Copy URL";

      



export const PublishFullTwo = (props?:PublishArgs) => {

 const {closed} = props || DEFAULT_ARGS;
 

    return `
    <publish-page-two title="${STR_TITLE}" subtitle="${STR_SUBTITLE}">
        <placeholder-img slot="animation"></placeholder-img>
        <div slot="button-collection">
        <rectangle-button label="${STR_SHARE}" imgrighthidden=true size="${STR_MEDIUM} color="${STR_WHITE}" path="${STR_ICNSHARE}" imglefthidden=false bold=false italic=false></rectangle-button>
        ${Rectangle({contents:STR_SHARE, size:STR_MEDIUM,color:STR_WHITE, bold:false, italic:false })}
        
        </div>
        <publish-dropdown ${closed && 'closed'} title="${STR_DROPDOWNTITLE}" slot="dropdown">
          <icon-wtext path="${STR_STUDENT}" text="${STR_SHARESTUDENT}"></icon-wtext>
          <icon-wtext icon="${STR_URLICON}" text="${STR_URL}"></icon-wtext>
        </publish-dropdown>
        <div slot="button-collection">${Rectangle({contents:STR_CREATE, size:STR_MEDIUM,color:STR_WHITE,  bold:false, italic:false  })}</div>
        <div slot="button-collection">${Rectangle({contents:STR_PLAY, size:STR_MEDIUM,color:STR_WHITE, bold:false, italic:false  })}
    
        </div>

        </publish-page-two>
    
    `
}

PublishFullTwo.args = DEFAULT_ARGS;
