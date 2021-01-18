import "@elements/lists/pill";
import "@elements/inputs/checkbox";



export default {
  title: 'Lists/Pill',
}

interface PillArgs {
label:string,
negative:boolean


}

const DEFAULT_ARGS:PillArgs = {
label:"School",
negative:false,

}

export const PillListItem = (props?:PillArgs) => {

  const {label, negative} = props || DEFAULT_ARGS;

    return `
       <pill-listitem label=${label} ${negative && 'negative'}></pill-listitem>
    `;
}

PillListItem.args = DEFAULT_ARGS
