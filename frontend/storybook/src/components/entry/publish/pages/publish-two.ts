import "@elements/entry/jig/publish/page-2";
import "@elements/widgets/tags/icon";
import "@elements/entry/jig/publish/publish-dropdown";
import "@elements/entry/jig/publish/publish-embed";
import { Rectangle } from "~/components/core/buttons/rectangle";
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

    
export const PublishFullTwo = (props?:PublishArgs) => {

 const {closed, embedclosed} = props || DEFAULT_ARGS;
 

    return `
    <publish-page-two>
        <div slot="button-collection">
        
        ${Rectangle({contents:STR_SHARE, size:"medium",color:"white", bold:false, italic:false, iconBefore:"share"})}
        
        </div>
        <publish-dropdown ${closed && 'closed'} slot="dropdown">
        <tooltip-right kind="success" slot="tooltip">${STR_TOOLTIP}</tooltip-right>

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
