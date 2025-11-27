
// simplest, modern way
document.addEventListener("DOMContentLoaded", main());

// loads later, after images, CSS
window.addEventListener("load", main());

function main() {
    console.log('test ready');
}
