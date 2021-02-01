import "@elements/entry/home/sections/header-section";
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
  const PATHHOME="icn-menu-home.svg";
  const PATHCONTENT="Icn_Menu_Content.svg";
  const PATHCREATE="Icn_Menu_Create.svg";
  const PATHCOMMUNITY="Icn_Menu_Community.svg";
  const PATHCLASSROOM="Icn_Menu_Classroom.svg";
  const PATHABOUTJI="Icn_Menu_About.svg";
 
 
export const HeaderParagraph = () => {
    return `

    <header-section>
<div slot="menu-tab">
 ${MenuTab({contents:STR_HOME,path:PATHHOME})}
</div>
<div slot="menu-tab">
 ${MenuTab({contents:STR_CONTENT,path:PATHCONTENT})}
</div>
<div slot="menu-tab">
 ${MenuTab({contents:STR_CREATE,path:PATHCREATE})}
</div>
<div slot="menu-tab">
 ${MenuTab({contents:STR_COMMUNITY,path:PATHCOMMUNITY})}
</div>
<div slot="menu-tab">
 ${MenuTab({contents:STR_CLASSROOM,path:PATHCLASSROOM})}
</div>
<div slot="menu-tab">
 ${MenuTab({contents:STR_ABOUTJI,path:PATHABOUTJI})}
</div>

<div slot="icon">
${Icon({label:STR_STUDENTCODE,icon:"group"})}

</div>


</header-section>
    `
}