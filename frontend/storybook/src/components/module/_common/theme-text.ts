import {ThemeId, ThemeControl} from "~/components/module/_common/theme";
import "@elements/mock/text-example";
import "@elements/core/module-page/grid-resize";
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
    text: "hello שָׁלוֹם",
}

export const TextExample = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {theme, text} = props;

    return `

      <module-page-grid-resize>
        <div slot="main">
        ${renderThemeBg(theme)}
            <div style="position: absolute; top: 0; left: 0;  width: 100%; height: 100%">
            ${makeLine("h1", theme, text)}
            ${makeLine("h2", theme, text)}
            ${makeLine("p1", theme, text)}
            ${makeLine("p2", theme, text)}
            </div>
        </div>
      </module-page-grid-resize>
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

//like render_theme_bg in Rust
function renderThemeBg(theme:ThemeId) {
	return `<img-ui style="position: absolute; top: 0; left: 0; display: block; width: 100%; height: 100%" path="theme/${theme as string}/bg.jpg"></img-ui>`
}
TextExample.args = DEFAULT_ARGS;
TextExample.argTypes = {
    theme: ThemeControl
}
