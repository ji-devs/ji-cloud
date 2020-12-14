import "@elements/buttons/circle-button";
import "@elements/nav/steps-nav";
export default {
  title: 'Steps Nav',
}

export const StepsNav = () => {
    return `
        <steps-nav steps="3">
          <circle-button slot="btn-1" text="1" label="Inactive"></circle-button>
          <circle-button slot="btn-2" text="2" label="Active" active></circle-button>
          <circle-button slot="btn-3" text="3" label="Disabled" disabled></circle-button>
        </steps-nav>
    `
}


export const StepsNavContained = () => {
    return `
      <div style="width: 500px">
        ${StepsNav()}
      </div>
    `
}