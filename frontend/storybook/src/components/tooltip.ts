import "@elements/tooltip/tooltip-top";
export default {
  title: 'Tooltip',
}

interface TooltipArgs {
  label: string,
  type:string,
  hidden: boolean,
  path:string,

}

const DEFAULT_ARGS:TooltipArgs = {
  label: "Please fill the missing fields",
  type:"error",
  hidden:false,
  path:"group-12812.svg"
}

export const TooltipTop = (props?:TooltipArgs) => {

  const {path, label, hidden, type} = props || DEFAULT_ARGS;
  
   return `<tooltip-top label="${label}" type="${type}" ${hidden && "hidden"} path="${path}"></tooltip-top>`
}




TooltipTop.args = DEFAULT_ARGS;
