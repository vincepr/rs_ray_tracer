// import * as ComLink from "comlink";


// async function spawnRenderer(start, end) {
//     const worker = new Worker("./worker.js");
//     const renderer = ComLink.wrap(worker);

//     console.log("worker initializing.", start, end);
//     await renderer.init(size, {start: start, end: end});

//     console.log("worker starting, beginning rendering.", start, end)
//     while ((lineData = await renderer.renderNext()) != false) {
//         // render one line
//         context.putImageData(lineData.data, 0, result.y);
//     }
// }

// // import('../pkg/index.js')
// //     .then(wasm => {
// //         const canvas = document.getElementById('drawing');
// //         const ctx = canvas.getContext('2d');

// //         const input_width = document.getElementById('input_width');
// //         const input_height = document.getElementById('input_height');
// //         const renderBtn = document.getElementById('render');
// //         const time_result = document.getElementById('time_result');

// //         renderBtn.addEventListener('click', () => {
// //             const width = parseInt(input_width.value) || 0;
// //             const height = parseInt(input_height.value) || 0;
// //             canvas.width = width;
// //             canvas.height = height;
// //             const start = new Date();

// //             (async () => {
// //                 spawnRenderer(0, size);
// //             })

// //             wasm.draw(ctx, width, height, "generatorstring");
// //             const time_ms = new Date().getTime() - start.getTime();
// //             time_result.innerHTML = `it took ${Math.round(time_ms/1000)}s to render`;
// //         });

// //         // wasm.draw(ctx, 50, 50, "generatorstring");
// //         const drawImage = (url) => {
// //             const image = new Image();
// //             image.src = url;
// //             image.onload = () => {
// //                 canvas.width = image.width;
// //                 canvas.height = image.height;
// //                ctx.drawImage(image, 0, 0);
// //             }
// //         }
// //         drawImage("./example.png");
// //     })
// //     .catch(console.error);

// const size = 2048;
// const canvas = document.getElementById('drawing');
// canvas.setAttribute("width", size);
// canvas.setAttribute("height", size);

// // double pixels to help against aliassing
// canvas.style.width = size / 2 + "px";
// canvas.style.height = size / 2 + "px";

// const context = canvas.getContext("2d");



// (async () => {
//     spawnRenderer(0, size);
// })

import * as Comlink from "comlink";
const size = 20;

const canvas = document.createElement("canvas");
canvas.setAttribute("width", size);
canvas.setAttribute("height", size);
canvas.style.width = size / 2 + "px";
canvas.style.height = size / 2 + "px";

document.body.appendChild(canvas);

const context = canvas.getContext("2d");

async function spawnRenderer(start, end) {
  const worker = new Worker('./worker.js');
  const renderer = Comlink.wrap(worker);
  
  console.log("initializing ", start, "-", end);
  await renderer.init(size, { start: start, end: end });

  console.log("rendering ", start, "-", end);
  let result;
  while ((result = await renderer.renderNext()) !== false) {
    context.putImageData(result.data, 0, result.y);
  }
}

(async () => {
  const workers = navigator.hardwareConcurrency || 4;
  const perWorker = size / workers;
  for (let i = 0; i < workers; i++) {
    spawnRenderer(i * perWorker, (i + 1) * perWorker);
  }
})();