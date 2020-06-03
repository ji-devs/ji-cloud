export const header = (loggedIn) => `
<header class="fancy">
  <div class="flex mb-4">
    <div class="cell">
        One of three columns
    </div>
    <div class="cell bg-indigo-100">
        <div class="green text-4xl">Two of three columns</div>
    </div>
    <div class="cell">
        ${loggedIn 
            ? `Profile`
            : `Sign in`
         }
    </div>
  </div>
</header>
`;