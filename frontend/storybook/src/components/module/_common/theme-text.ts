import {ThemeId, ThemeControl} from "~/components/module/_common/theme";
export default {
    title: "Module / _common"
}

interface Args {
    theme: ThemeId;
}

const DEFAULT_ARGS:Args = {
    theme: "blank"
}

type ElementKind = "h1" | "h2" | "p1" | "p2";

export const TextExample = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {theme} = props;

    return `
	Note: font-families aren't rendered because the font loading is unicode-range specific and dealt with in Rust only at the moment
	<p>&nbsp;</p>
        <div>
		${makeLine("h1", theme)}
		${makeLine("h2", theme)}
		${makeLine("p1", theme)}
		${makeLine("p2", theme)}
        </div>
    `;
}

function makeLine(element: ElementKind, theme: ThemeId) {
	return `
		<b>${element.toUpperCase()}</b>:
		<br/>
		<div style="${makeStyle(element, theme)}">Hello World</div>
	`;
}

function makeStyle(element: ElementKind, theme: ThemeId) {
	let style = `font-family: var(--theme-${theme}-${element}-font-family);`;
	style += ` font-size: var(--theme-${theme}-${element}-font-size);`;
	style += ` color: var(--theme-${theme}-${element}-color);`;

	return(style);
}

TextExample.args = DEFAULT_ARGS;
TextExample.argTypes = {
    theme: ThemeControl
}
