const cp = require('child_process');
cp.exec('cargo run --bin codeviz-cli -- run --path codeviz-cli/src --diagram module', (err, stdout, stderr) => {
    console.log("stdout: ", stdout);
    console.log("stderr: ", stderr);
});
