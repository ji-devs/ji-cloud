import {renderTemplate as tmpl} from "@utils/template";
import {setTextId} from "@utils/dom";
import {} from "@utils/dom";
import profile from "@templates/user/profile.html";

export default {
  title: 'User/Profile',
}

export const Profile = () => {
    const page = tmpl(profile);

    setTextId(page, "profile", "raw profile info here");

    return page;
}