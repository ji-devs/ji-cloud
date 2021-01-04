import "@elements/admin/templates-layout/publish-full";
import "@elements/image-thumbnail";
import "@elements/inputs/textarea-text";


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
        label: "Description"

      }



export const PublishFullOne = (props?:PublishArgs) => {

    const {title, subtitle, path, name, helpertext, errormessage,error, instruction, label } = props || DEFAULT_ARGS;


    return `
    <publish-full title="${title}" subtitle="${subtitle}">
        <image-thumbnail path="${path}" slot="column_one"></image-thumbnail>
        <input-text slot="column_two" label="${name}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <textarea-text label="${label}" slot="column_two"></textarea-text>
        <dropdown-select slot="topleft" path="${dropdownicon}" label="${language}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <dropdown-select slot="topright" path="${dropdownicon}" label="${age}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <dropdown-select slot="bottomleft" path="${dropdownicon}" label="${goal}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <dropdown-select slot="bottomright" path="${dropdownicon}" label="${categories}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>    </publish-full>
    
    `
}

PublishFullOne.args = DEFAULT_ARGS;

