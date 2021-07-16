import {ThemeId, ThemeControl} from "~/components/module/_common/theme";
import "@elements/mock/text-example";
import {Variant} from "@elements/mock/text-example";

export default {
    title: "Module / _common"
}

interface Args {
    theme: ThemeId,
    text: string,
}

const DEFAULT_ARGS:Args = {
    theme: "blank",
    text: "Hello World",
}

export const TextExample = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {theme, text} = props;

    return `
        <div>
		${makeLine("h1", theme, text)}
		${makeLine("h2", theme, text)}
		${makeLine("p1", theme, text)}
		${makeLine("p2", theme, text)}
        </div>
    `;
}

const Foo = () => {
}

function makeLine(variant: Variant, theme: ThemeId, text:string) {
	return `
		<b>${variant.toUpperCase()}</b>:
		<br/>
		<mock-text-example variant="${variant}" theme="${theme as string}" text="${text}"></mock-text-example>
	`;
}

TextExample.args = DEFAULT_ARGS;
TextExample.argTypes = {
    theme: ThemeControl
}
