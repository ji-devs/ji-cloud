import "@elements/module-page/grid-resize";
import "@elements/legacy/sidebar";
import "@elements/legacy/sidebar-module";
import "@elements/legacy/main";
import "@elements/images/legacy";
import {withSlot} from "@utils/dom";
import {arrayIndex, mapToString} from "@utils/array";
export default {
  title: 'Legacy/Shell',
}

interface ShellArgs {
    nModules: number,
    selected: number,
}

const DEFAULT_SHELL_ARGS:ShellArgs = {
    nModules: 3,
    selected: 0,
};

export const Shell = (props?:ShellArgs) => {

    const {nModules, selected} = props || DEFAULT_SHELL_ARGS;

    return `
      <module-page-grid-resize legacy>
        <legacy-sidebar slot="sidebar" nModules="${nModules}">
        ${mapToString(
            arrayIndex(nModules), 
            index => {
                return `
                <legacy-sidebar-module 
                    slot="module-${index}" 
                    index="${index}"
                    jigId="web-stress-test"
                    moduleId="21f7c750-0623-11ea-beec-3f00dc406aac"
                    ${index === selected ? "selected": ""}
                    >
                    <img-legacy
                        slot="img" 
                        jigId="web-stress-test"
                        moduleId="21f7c750-0623-11ea-beec-3f00dc406aac"
                        path="photoThumb1.jpg"
                        mock>
                    </img-legacy>

                </legacy-sidebar-module>`
            }
        )}
        </legacy-sidebar>
        <legacy-main slot="main"></legacy-main>
      </module-page-grid-resize>
     `
}

Shell.args = DEFAULT_SHELL_ARGS;
