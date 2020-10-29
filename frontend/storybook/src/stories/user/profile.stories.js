import {renderTemplate as tmpl} from "@utils/template";
import {setTextId} from "@utils/dom";
import {} from "@utils/dom";
import profile from "@templates/user/profile/profile.html";
import emailChange from "@templates/user/profile/email-change.html";
export default {
  title: 'User/Profile',
}

export const Profile = () => {
    const page = tmpl(profile);

    setTextId(page, "profile", "raw profile info here");

    return page;
}


export const EmailChange = () => {
    const page = tmpl(emailChange);
    return page;
}