import {argsToAttrs} from "@utils/attributes";
import {Header as CommonHeader} from "~/components/module/_common/header";

export default {
    title: "Module / Memory / Edit / Steps / Sections"
}

const STR_TITLE = "Create a Memory Game";

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Header = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return CommonHeader({title: STR_TITLE});
}

Header.args = DEFAULT_ARGS;
