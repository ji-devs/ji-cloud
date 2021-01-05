import "@elements/admin/templates-layout/login-full";
import "@elements/titles/underlined-title";
import "@elements/titles/plain-blue";
import "@elements/dividers/or-divider";
import "@elements/cards/grey-card";
import "@elements/lists/vertical-checkbox-list";
import {AgeTwo} from "~/components/lists/age-two";
import {AGETWO_OPTIONS} from "~/mock/meta";



export default {
  title: 'Full Pages/Login',
  
}

interface UserInfoArgs {


  }

  const DEFAULT_ARGS:UserInfoArgs = {

  }

export const UserInfo = (props?:UserInfoArgs) => {



    return `
    <grey-card>
       <div slot="content">${AgeTwo()}</div>
    </grey-card>
    
    `
}

UserInfo.args = DEFAULT_ARGS;
