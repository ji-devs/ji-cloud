export const REMOTE_STATIC = process.env["NODE_ENV"] === "development" 
        ? 'http://localhost:4102'
        : "https://storage.googleapis.com/ji-cloud-eu";

export const REMOTE_UI = `${REMOTE_STATIC}/app/ui`;

export const Path = (() => {

    return {
        ui: path => `${REMOTE_UI}/${path}`,
    }
})();
