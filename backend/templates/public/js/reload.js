/* 
    Simple script to reload the webpage on server change
    The epoch value is set on the server each time its recompiled
    Which happens on file change
*/

//Tune this for how often to poll
const INTERVAL = 500;

let _epoch;

function ping() {
    fetch("/epoch")
        .then(data => data.text())
        .then(epoch => {
            if(_epoch != null && _epoch != epoch) {
                console.log("reloading...");
                window.location.reload();
            } else {
                _epoch = epoch;
                setTimeout(ping, INTERVAL);
            }
        });
}

ping();