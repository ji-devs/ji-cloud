import "@elements/buttons/circle-button";
import "@elements/nav/steps-nav";
import {arrayCount} from "@utils/array";
export default {
  title: 'Steps Nav',
}

export const StepsNav = ({nSteps}) => {
    return `
        <steps-nav steps="${nSteps}">
          ${arrayCount(nSteps)
            .map(x => `<circle-button slot="btn-${x}" text="${x}" label="Step ${x}"></circle-button>`)
          }
        </steps-nav>
    `
}

export const StepsNavContained = ({nSteps, width}) => {
    return `
      <div style="width: ${width}px">
        ${StepsNav({nSteps})}
      </div>
    `
}

StepsNav.args = {
  nSteps: 3
}
StepsNavContained.args = {
  nSteps: 3,
  width: 500
}