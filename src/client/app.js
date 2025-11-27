
// simplest, modern way
document.addEventListener("DOMContentLoaded", main());

// loads later, after images, CSS
window.addEventListener("load", main2());

function main() {
    console.log('test ready');
}

function main2() {
    console.log("test2 ready");
}
