const {SecretManagerServiceClient} = require('@google-cloud/secret-manager');
const client = new SecretManagerServiceClient();

const getSecret = (name) =>
    client.initialize()
        .then(() => new Promise((resolve, reject) => {
            client.getProjectId((_, projectId) => {
                resolve(projectId);
            });
        }))
        .then(projectId => `projects/${projectId}/secrets/${name}/versions/latest`)
        .then(secretName => client.accessSecretVersion({name: secretName}))
        .then(([version]) => version.payload.data.toString('utf8'))
        //.then(result => (console.log(result), result));

module.exports = {
    getSecret
}
