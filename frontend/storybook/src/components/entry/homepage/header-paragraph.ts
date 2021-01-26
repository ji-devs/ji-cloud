import "@elements/entry/home/sections/header-section";
import icon from "~/components/core/cards/icon";
   import {MenuTab} from "~/components/core/menu/menu-tab";
   import {Icon} from "~/components/core/cards/icon";


export default {
  title: 'Homepage',
}

  const STR_HOME="Home";
  const STR_CONTENT="Content";
  const STR_CREATE="Create";
  const STR_COMMUNITY="Community";
  const STR_CLASSROOM="Classroom";
  const STR_ABOUTJI="About JI";
  const STR_STUDENTCODE="Student Code";
  const STR_PATHHOME="icn-menu-home.svg";
  const STR_PATHCONTENT="Icn_Menu_Content.svg";
  const STR_PATHCREATE="Icn_Menu_Create.svg";
  const STR_PATHCOMMUNITY="Icn_Menu_Community.svg";
  const STR_PATHCLASSROOM="Icn_Menu_Classroom.svg";
  const STR_PATHABOUTJI="Icn_Menu_About.svg";
 
 
export const HeaderParagraph = () => {
    return `

    <header-section>
<div slot="menu-tab">
 ${MenuTab({contents:STR_HOME,path:STR_PATHHOME})}
</div>
<div slot="menu-tab">
 ${MenuTab({contents:STR_CONTENT,path:STR_PATHCONTENT})}
</div>
<div slot="menu-tab">
 ${MenuTab({contents:STR_CREATE,path:STR_PATHCREATE})}
</div>
<div slot="menu-tab">
 ${MenuTab({contents:STR_COMMUNITY,path:STR_PATHCOMMUNITY})}
</div>
<div slot="menu-tab">
 ${MenuTab({contents:STR_CLASSROOM,path:STR_PATHCLASSROOM})}
</div>
<div slot="menu-tab">
 ${MenuTab({contents:STR_ABOUTJI,path:STR_PATHABOUTJI})}
</div>

<div slot="icon">
${Icon({label:STR_STUDENTCODE,icon:"group"})}

</div>


</header-section>
    `
}