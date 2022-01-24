// (async () => {
//     let response = await fetch('http://localhost:3000/rust_project_bg.wasm');
//     let bytes = await response.arrayBuffer();
//     let { instance } = await WebAssembly.instantiate(bytes, { });

//     const {get_string, get_num} = instance.exports;

//     console.log(get_string("rust", instance.exports));
//     console.log(get_num(4));
//   })(

import init, {get_num, get_string, get_data} from "./rust_project.js"

(async () => {
    await init("http://localhost:3000/rust_project_bg.wasm")

    console.log(get_num(3));
    console.log(get_string("rust"));
    const {results} = await get_data("https://dbpedia.org/sparql", "select distinct ?Concept where {[] a ?Concept} LIMIT 50")
    const {bindings} = results
    bindings.map(binding => console.log(binding.Concept.value))
})()

