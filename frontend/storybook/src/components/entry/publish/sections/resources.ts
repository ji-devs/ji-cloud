<<<<<<< HEAD
import {argsToAttrs} from "@utils/attributes";
=======
>>>>>>> naomi/master
import "@elements/entry/jig/publish/resources";

export default {
  title: 'Entry/Jig / Publish / Sections',
}

<<<<<<< HEAD


interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Resources = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

=======
export const Resources = () => {
  
>>>>>>> naomi/master
    return `
        <resources-column></resources-column>
    `
}

<<<<<<< HEAD
Resources.args = DEFAULT_ARGS;
=======
>>>>>>> naomi/master
