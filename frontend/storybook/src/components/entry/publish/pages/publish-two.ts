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

      const STR_SHARE = "share the JIG";
      const STR_CREATE = "create a new JIG";
      const STR_PLAY = "play the JIG";
      const STR_TOOLTIP = "Copied to the clipboard";
      const STR_DROPDOWNTITLE = "Select Share Option";
    


export const PublishFullTwo = (props?:PublishArgs) => {

 const {closed, embedclosed} = props || DEFAULT_ARGS;
 

    return `
    <publish-page-two>
        <placeholder-img slot="animation"></placeholder-img>
        <div slot="button-collection">
        
        ${Rectangle({contents:STR_SHARE, size:"medium",color:"white", bold:false, italic:false, iconBefore:"share"})}
        
        </div>
        <publish-dropdown ${closed && 'closed'} title="${STR_DROPDOWNTITLE}" slot="dropdown">
          <dropdown-list-object  slot="list" mode="share"></dropdown-list-object>
          <dropdown-list-object slot="list" mode="embed"></dropdown-list-object>
          <dropdown-list-object slot="list" mode="link">
          <tooltip-right kind="success" slot="tooltip">${STR_TOOLTIP}</tooltip-right>
          </dropdown-list-object>
          
        </publish-dropdown>
        <publish-embed slot="dropdown" ${embedclosed && "closed"}>
          <tooltip-right kind="success" slot="tooltip">${STR_TOOLTIP}</tooltip-right>

        </publish-embed>
        <div slot="button-collection">${Rectangle({contents:STR_CREATE, size:"medium",color:"white",  bold:false, italic:false, iconBefore:"create"})}</div>
        <div slot="button-collection">${Rectangle({contents:STR_PLAY, size:"medium",color:"white", bold:false, italic:false, iconBefore:"play"  })}
    
        </div>

        </publish-page-two>
    
    `
}

PublishFullTwo.args = DEFAULT_ARGS;
