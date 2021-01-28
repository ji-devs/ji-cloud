import "@elements/core/pills/pill-close";
import "@elements/core/inputs/checkbox";



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
       <pill-close label=${label} ${negative && 'negative'}></pill-close>
    `;
}

PillListItem.args = DEFAULT_ARGS
