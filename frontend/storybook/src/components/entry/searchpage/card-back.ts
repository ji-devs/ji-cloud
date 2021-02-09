import "@elements/core/cards/parts/image";
import "@elements/core/dividers/horizontal-full";
import "@elements/entry/search/card-back";
import "@elements/widgets/tags/icon"; 
import "@elements/core/cards/parts/dropdown";
import "@elements/core/pills/pill-close";
import "@elements/core/titles/ji";

export default {
  title: 'Entry / Home / Search',
}



interface CardBackArgs {
  
    collapsed:boolean,
    
   
   }
   
   const DEFAULT_ARGS:CardBackArgs = {
   
    collapsed:false,
     
   }

const STR_BEREISHIT = "Bereishit Stories, Briat Haolam Story, Adam veChava"
const STR_TWOWEEKS = "2 weeks ago";
const STR_HEBREW = "Hebrew letters"
const STR_DESCRIPTION ="Description";
const STR_DESCRIPTIONPARA = "This game is about… using … Lorem Ipsum is simply dummy text of the printing and typesetting industry";
const STR_RESOURCES = "Additional resources";
const STR_LESSONPLAN = "Lesson plan";
const STR_CURRICULUM = "Curriculum";
const STR_EXPLINATION = "Explination";
const STR_ACTIVITIES ="Activities Suggestion"
const STR_SEEJIG = "See Anat’s JIGs";
const STR_NAME ="Ji Team";
const STR_ANAT = "Anat";
 
export const SearchCardBack = (props?:Partial<CardBackArgs>) => {

    const {collapsed} = props || DEFAULT_ARGS;

    return `<card-back label="Placeholder" color="blue" size="medium" fontweight="" back>
       <title-ji color="white" weight="bold" slot="title">${STR_BEREISHIT}</title-ji>
       <tag-icon slot="subtitle" kind="time" label="${STR_TWOWEEKS}"></tag-icon>
       <card-dropdown slot="dropdowns" ${collapsed && "collapsed"} pill>
            <pill-close label=${STR_HEBREW } negative slot="title"></pill-close>
            <pill-close label=${STR_HEBREW } negative slot="title"></pill-close>
            <pill-close label=${STR_HEBREW } negative slot="title"></pill-close>
            <pill-close label=${STR_HEBREW } negative slot="title"></pill-close>
            <pill-close label=${STR_HEBREW } negative slot="title"></pill-close>


       </card-dropdown>
       <card-dropdown slot="dropdowns" ${collapsed && "collapsed"}>
       <title-ji color="white" size="small" slot="title">${STR_DESCRIPTION}</title-ji>
       <title-ji color="white" weight="light" size="small" slot="content">${STR_DESCRIPTIONPARA}</title-ji>
       </card-dropdown>
       <card-dropdown slot="dropdowns" ${collapsed && "collapsed"}>
       <title-ji size="small" color="white" slot="title">${STR_RESOURCES}</title-ji>
       <tag-icon slot="content" kind="file" label="${STR_LESSONPLAN}"></tag-icon>
       <tag-icon slot="content" kind="file" label="${STR_CURRICULUM}"></tag-icon>
       <tag-icon slot="content" kind="file" label="${STR_EXPLINATION}"></tag-icon>
       <tag-icon slot="content" kind="file" label="${STR_ACTIVITIES}"></tag-icon>

       </card-dropdown>
       <tag-icon slot="user" kind="jiwhite" label="${STR_NAME}"></tag-icon>
       <title-ji color="white" slot="username" size="small">${STR_ANAT}</title-ji>

       <title-ji color="white" slot="jigs" size="small" weight="normal">${STR_SEEJIG}</title-ji>

    </card-back>
    `
}



   SearchCardBack.args = DEFAULT_ARGS;
