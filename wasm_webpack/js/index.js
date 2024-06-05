import * as Comlink from "comlink";

setupCanvasOnPageload("./example.png");
setupTexareaOnPageload("./book_cover.yaml")
setupPage();

function setupPage() {
  const renderBtn = document.getElementById('render');
  renderBtn.addEventListener('click', handleRenderBtnClicked);
}

async function handleRenderBtnClicked(){
  // start timer
  const start_time = new Date();

  let coreCount = document.getElementById('input_cores').value;
  coreCount = coreCount > 0 ? coreCount : null;

  // prepare our canvas/ctx
  const canvas = document.getElementById('drawing');
  const ctx = canvas.getContext('2d');
  const yaml_str = document.getElementById('input_yaml').value;

  let {width, height} = await parseYamlForSceneData();
  canvas.width = width;
  canvas.height = height;

  // start the rendering
  await startParallelRendering(ctx, yaml_str, height, coreCount);

  // display timer
  const time_ms = new Date().getTime() - start_time.getTime();
  time_result.innerHTML = `it took ${Math.round(time_ms/1000)}s to render`;
}

// Parse in yaml in wasm one time, to get width and height.
// We could also check for (syntax-)errors here. For better error-feedback.
async function parseYamlForSceneData() {
  const yaml_str = document.getElementById('input_yaml').value;
  const worker = new Worker('./worker.js');
  const renderer = Comlink.wrap(worker);
  await renderer.init({ start: 0, end: 0, yaml_str: yaml_str });
  return await renderer.getWidthHeight();
}

async function startParallelRendering(ctx, yaml_str, sceneHeight, fixedNrCores = null) {
  async function spawnRenderer(start, end) {
    const worker = new Worker('./worker.js');
    const renderer = Comlink.wrap(worker);
    
    await renderer.init({ start: start, end: end, yaml_str: yaml_str });

    console.log("starting worker for ", start, " to ", end);
    let result;
    while ((result = await renderer.renderNext()) !== false) {
      ctx.putImageData(result.imgData, 0, result.y);
    }
  }

  const possibleWorkers = navigator.hardwareConcurrency || 4;
  const workers = fixedNrCores !== null ? fixedNrCores : possibleWorkers;
  console.log(`found ${navigator.hardwareConcurrency} cores, multithreading with ${workers} concurrent workers.`)
  const perWorker = sceneHeight / workers;

  let tasks = [];
  for (let i = 0; i < workers; i++) {
    tasks.push(spawnRenderer(i * perWorker, (i + 1) * perWorker));
  }

  await Promise.all(tasks);
}

function setupCanvasOnPageload(url) {
  const canvas = document.getElementById('drawing');
  const ctx = canvas.getContext('2d');
  const image = new Image();
  image.src = url;
  image.onload = () => {
      canvas.width = image.width;
      canvas.height = image.height;
      ctx.drawImage(image, 0, 0);
  }
}

async function setupTexareaOnPageload(path) {
  fetch(path)
    .then(response => response.text())
    .then((text) => {
      const textArea = document.getElementById("input_yaml");
      textArea.value = text
    })
}