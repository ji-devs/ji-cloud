import "@elements/widgets/module-page/grid-resize";
import "@elements/widgets/module-page/grid-plain";
import "@elements/widgets/module-page/iframe";
import {withSlot} from "@utils/dom";

export default {
  title: 'Widgets / Module Page',
}

export const GridResize = ({sidebar, header, main, footer}) => {
    return `
      <module-page-grid-resize>
        ${withSlot("sidebar", sidebar)}
        ${withSlot("header", header)}
        ${withSlot("main", main)}
        ${withSlot("footer", footer)}
      </module-page-grid-resize>
     `
}

export const GridResizeScrollable = ({sidebar, header, main, footer}) => {
    return `
      <module-page-grid-resize scrollable>
        ${withSlot("sidebar", sidebar)}
        ${withSlot("header", header)}
        ${withSlot("main", main)}
        ${withSlot("footer", footer)}
      </module-page-grid-resize>
     `
}
export const GridPlain = ({sidebar, header, main, footer}) => {
    return `
      <module-page-grid-plain>
        ${withSlot("sidebar", sidebar)}
        ${withSlot("header", header)}
        ${withSlot("main", main)}
        ${withSlot("footer", footer)}
      </module-page-grid-plain>
     `
}
export const Iframe = ({main}) => {
    return `
      <module-page-iframe>
        ${withSlot("main", main)}
      </module-page-iframe>
     `
}
export const GridEditPreview = ({header, main}) => {
    return `
      <module-page-grid-resize>
        ${withSlot("header", header)}
        ${withSlot("main", main)}
      </module-page-grid-resize>
     `
}


const makeArgs = (mainFontSize:string) => ({
  sidebar: `<div style="background-color: yellow; height: 100%; text-align: center"><textarea>Module Sidebar </textarea></div>`,
  header: `<div style="background-color: red; color: white; text-align: center;"><textarea>Header</textarea></div>`,
  main: `<div style="background-color: green; width: 100%; height: 100%; display: flex; flex-direction: column; justify-content: space-between;">
          <div></div>
          <div style="color: white; font-size: ${mainFontSize}; width: 100%; text-align: center;">
            <textarea style="font-size: ${mainFontSize}">Main</textarea>
          </div>
          <div></div>
        </div>`,
  footer: `<div style="background-color: blue; color: white; text-align: center;"><textarea>Footer</textarea></div>`
});

const resizeArgs = makeArgs(`10rem`);
const plainArgs = makeArgs(`initial`);

GridResize.args = resizeArgs; 
GridResizeScrollable.args = resizeArgs; 
GridPlain.args = plainArgs; 
Iframe.args = {main: resizeArgs.main}; 
GridEditPreview.args = {main: resizeArgs.main, header: resizeArgs.header}; 