import "@elements/entry/jig/publish/page-2";
import "@elements/entry/home/TOSORT/image-thumbnail"; 
import "@elements/entry/home/TOSORT/placeholder"; 
import "@elements/widgets/tags/icon";
import "@elements/entry/jig/publish/publish-dropdown";
import "@elements/core/inputs/textarea";
import "@elements/core/dividers/spacer-fourty";
import "@elements/entry/jig/publish/dropdown-list-object";
import "@elements/entry/jig/publish/publish-embed";
import { Rectangle } from "~/components/core/buttons/rectangle";
import {Color, Size, IconAfter, IconBefore} from "@elements/core/buttons/rectangle";
import { colorStyles } from "@elements/_styles/colors";
import "@elements/widgets/tooltips/right";
export default {
  title: 'Entry/Jig/Publish/Pages',
}

  
  interface PublishArgs {
    closed:boolean,
    embedclosed:boolean,
  
    }

    const DEFAULT_ARGS:PublishArgs = {
       closed:false,
       embedclosed:false,
      }

      
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
      const STR_EMBED ="Embed this JIG";
      const STR_EMBEDICON ="Icn_Embed.svg";

      



export const PublishFullTwo = (props?:PublishArgs) => {

 const {closed, embedclosed} = props || DEFAULT_ARGS;
 

    return `
    <publish-page-two>
        <placeholder-img slot="animation"></placeholder-img>
        <div slot="button-collection">
        <rectangle-button label="${STR_SHARE}" imgrighthidden=true size="${STR_MEDIUM} color="${STR_WHITE}" path="${STR_ICNSHARE}" imglefthidden=false bold=false italic=false></rectangle-button>
        ${Rectangle({contents:STR_SHARE, size:STR_MEDIUM,color:STR_WHITE, bold:false, italic:false, iconBefore:"share"})}
        
        </div>
        <publish-dropdown ${closed && 'closed'} title="${STR_DROPDOWNTITLE}" slot="dropdown">
          <dropdown-list-object  slot="list" icon="${STR_STUDENT}" label="${STR_SHARESTUDENT}"></dropdown-list-object>
          <dropdown-list-object slot="list" icon="${STR_EMBEDICON}" label="${STR_EMBED}"></dropdown-list-object>
          <dropdown-list-object slot="list" icon="${STR_URLICON}" label="${STR_URL}">
          <tooltip-right kind="success" slot="tooltip">Copied to the clipboard</tooltip-right>
          </dropdown-list-object>
          
        </publish-dropdown>
        <publish-embed slot="dropdown" ${embedclosed && "closed"}>
          <tooltip-right kind="success" slot="tooltip">Copied to the clipboard</tooltip-right>

        </publish-embed>
        <div slot="button-collection">${Rectangle({contents:STR_CREATE, size:STR_MEDIUM,color:STR_WHITE,  bold:false, italic:false, iconBefore:"create "})}</div>
        <div slot="button-collection">${Rectangle({contents:STR_PLAY, size:STR_MEDIUM,color:STR_WHITE, bold:false, italic:false, iconBefore:"play"  })}
    
        </div>

        </publish-page-two>
    
    `
}

PublishFullTwo.args = DEFAULT_ARGS;
