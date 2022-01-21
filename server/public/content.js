// (async () => {
//     let response = await fetch('http://localhost:3000/rust_project_bg.wasm');
//     let bytes = await response.arrayBuffer();
//     let { instance } = await WebAssembly.instantiate(bytes, { });

//     const {get_string, get_num} = instance.exports;

//     console.log(get_string("rust", instance.exports));
//     console.log(get_num(4));
//   })(

import init, {get_string} from "./rust_project.js"

(async () => {
    await init("http://localhost:3000/rust_project_bg.wasm")

    console.log(get_string("hello, i am calling uou from js"));
})()

