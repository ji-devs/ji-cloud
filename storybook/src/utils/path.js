export const REMOTE_STATIC = process.env["NODE_ENV"] === "development" 
        ? 'http://localhost:4102'
        : "https://media.jicloud.org";

export const REMOTE_UI = `${REMOTE_STATIC}/app/ui`;

export const Path = (() => {

    return {
        ui: path => `${REMOTE_UI}/${path}`,
    }
})();
