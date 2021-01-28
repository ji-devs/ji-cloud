 import "@elements/entry/jig/publish/panel/jiggling-body";
import "@elements/entry/jig/publish/panel/side-bar-number";
import "@elements/entry/home/TOSORT/column-list";
import "@elements/entry/jig/publish/panel/jig-squad";
import "@elements/entry/jig/publish/panel/jig-panel";

 

   

export default {
  title: 'Homepage',
}

 const PATHEDIT="Edit - JIG panel.svg";
const STR_01="01";
const STR_COVER="Cover";

const PATHCOVER="Cover_small.svg";

const STR_02="02";
const STR_POSTER="Poster";

const STR_03="03";
const STR_MEMORY="Memory Game";

const STR_04="04";
const STR_FLASHCARDS="Flashcards";

const STR_05="05";
const STR_TAPPING="Tapping Board";

const STR_ENDING="Ending";
const STR_PUBLISH="Publish";


const PATHFEET1="Group 13365.svg";
const PATHFEET2="Group 13414.svg";
const PATHFEET3="Group 13374.svg";
const PATHFEET4="Path 148075.svg";
 



export const JIGpanel = () => {
    return `

<jig-panel>

    <jig-squad pathfeet="${PATHFEET1}" slot="jig-squad">   

    <div slot="side-bar-number">
    <side-bar-number path="${PATHCOVER}" title="${STR_01}">
    <column-list   text_line="${STR_COVER}"   slot="subtitle"  ></column-list>

    </side-bar-number>
    </div>
    <div slot="jiggling-body">
    <jiggling-body path="${PATHEDIT}"></jiggling-body>
    </div>
 
  </jig-squad>
  <jig-squad pathfeet="${PATHFEET2}"  slot="jig-squad">   

  <div slot="side-bar-number">
  <side-bar-number path="${PATHCOVER}" title="${STR_02}">
  <column-list   text_line="${STR_POSTER}"  slot="subtitle"  ></column-list>

  </side-bar-number>
  </div>
  <div slot="jiggling-body">
  <jiggling-body path="${PATHEDIT}"></jiggling-body>
  </div>

</jig-squad>


</jig-squad>
  <jig-squad pathfeet="${PATHFEET3}"  slot="jig-squad">   

  <div slot="side-bar-number">
  <side-bar-number path="${PATHCOVER}" title="${STR_03}">
  <column-list   text_line="${STR_MEMORY}"  slot="subtitle"  ></column-list>

  </side-bar-number>
  </div>
  <div slot="jiggling-body">
  <jiggling-body path="${PATHEDIT}"></jiggling-body>
  </div>

</jig-squad>


</jig-squad>
  <jig-squad pathfeet="${PATHFEET1}"  slot="jig-squad">   

  <div slot="side-bar-number">
  <side-bar-number path="${PATHCOVER}" title="${STR_04}">
  <column-list   text_line="${STR_FLASHCARDS}"  slot="subtitle"  ></column-list>

  </side-bar-number>
  </div>
  <div slot="jiggling-body">
  <jiggling-body path="${PATHEDIT}"></jiggling-body>
  </div>

</jig-squad>


</jig-squad>
  <jig-squad pathfeet="${PATHFEET2}"  slot="jig-squad">   

  <div slot="side-bar-number">
  <side-bar-number path="${PATHCOVER}" title="${STR_05}">
  <column-list   text_line="${STR_TAPPING}"  slot="subtitle"  ></column-list>

  </side-bar-number>
  </div>
  <div slot="jiggling-body">
  <jiggling-body path="${PATHEDIT}"></jiggling-body>
  </div>

</jig-squad>



</jig-squad>
  <jig-squad pathfeet="${PATHFEET4}"  slot="jig-squad">   

  <div slot="side-bar-number">
  <side-bar-number path="${PATHCOVER}" title="${STR_ENDING}">
  <column-list   text_line="${STR_PUBLISH}"  slot="subtitle"  ></column-list>

  </side-bar-number>
  </div>
  <div slot="jiggling-body">
  <jiggling-body path="${PATHEDIT}"></jiggling-body>
  </div>

</jig-squad>

</jig-panel>

     `
}
