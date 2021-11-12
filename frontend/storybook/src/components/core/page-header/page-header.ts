import "@elements/core/page-header/page-header";
import "@elements/core/page-header/page-header-link";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Core / Page Header",
};

interface Args {
    loggedIn: boolean;
}

const DEFAULT_ARGS: Args = {
    loggedIn: false,
};

export const PageHeader = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <page-header ${argsToAttrs(props)}>
            <page-header-link slot="links" kind="home"></page-header-link>
            <page-header-link slot="links" kind="content"></page-header-link>
            <page-header-link slot="links" kind="create"></page-header-link>
            <page-header-link slot="links" kind="community"></page-header-link>
            <page-header-link slot="links" active kind="classroom"></page-header-link>
            <page-header-link slot="links" kind="about"></page-header-link>
            <button-rect slot="donate" color="green" size="small" bold>Donate</button-rect>

            ${
                props.loggedIn
                    ? `
                <page-header-profile name="Someone" slot="user"></page-header-profile>
            `
                    : `
                <button-rect kind="text" slot="user">Sign up</button-rect>
                <button-rect kind="text" slot="user">Login</button-rect>
            `
            }

        </page-header>
    `;
};

PageHeader.args = DEFAULT_ARGS;
