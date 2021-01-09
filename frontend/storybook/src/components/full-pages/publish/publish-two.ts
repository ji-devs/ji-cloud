import "@elements/admin/templates-layout/publish-full-two";
import "@elements/image-thumbnail";
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
        title: string,
        subtitle: string,
        path: string,
        name: string,
        helpertext:string,
        errormessage: string,
        instruction: boolean,
        error: boolean,
        label:string,
        dropdownicon:string,
        language: string,
        age:string,
        goal:string,
        categories:string,
        title_two: string,
        bold: boolean,
        icontitle_one:string,
        path_two:string,
        label_button:string,
        color:string,
        size: string,
        uploaded:boolean,
        btn_one:string,
        label_btn_one:string,
        btn_two:string,
        label_btn_two:string,
        btn_three:string,
        label_btn_three:string,

     
  
    
    }

    const DEFAULT_ARGS:PublishArgs = {
        title: "Settings and JIG info.",
        subtitle: "Last step before publishing",
        path: "red-sea-book.png",
        name: "JIG’s name",
        helpertext: "", 
        errormessage: "",
        instruction: false,
        error: false,
        label: "Description",
        dropdownicon:"icn-chevron-dropdown-up.svg",
        language: "Language of instructions",
        age: "Age",
        goal: "Teaching Goal",
        categories: "Categories",
        title_two:"Additional resources (Optional)",
        bold: false,
        icontitle_one: "Test",
        path_two:"Icn_CheckMark.svg",
        label_button:"Publish JIG",
        size:"medium",
        color:"white",
        uploaded:false,
        btn_one:"",
        label_btn_one:"share the JIG",
        btn_two:"",
        label_btn_two:"create a new JIG",
        btn_three:"icn-video-activity-hover.svg",
        label_btn_three:"play the JIG",


      }



export const PublishFullTwo = (props?:PublishArgs) => {

 const {title, size, color,subtitle, label_btn_one,btn_one,label_btn_two,btn_two,label_btn_three,btn_three } = props || DEFAULT_ARGS;
 

    return `
    <publish-full-two title="${title}" subtitle="${subtitle}">
       
        <div slot="button-collection">${RectangleButton({label:label_btn_one, size:size,color:color, path:btn_one })}</div>
        <div slot="button-collection">${RectangleButton({label:label_btn_two, size:size,color:color, path:btn_two })}</div>
        <div slot="button-collection">${RectangleButton({label:label_btn_three, size:size,color:color, path:btn_three })}</div>

        </publish-full-two>
    
    `
}

PublishFullTwo.args = DEFAULT_ARGS;
