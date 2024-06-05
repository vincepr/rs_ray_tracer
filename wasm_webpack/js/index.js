import * as Comlink from "comlink";

setupInitialCanvas("./example.png");
loadTextareaFromFile("./cover.yaml")
setupPage();

function setupPage() {
  const renderBtn = document.getElementById('render');
  renderBtn.addEventListener('click', handleRenderBtnClicked);
}

//
//                  Rendering
//

async function handleRenderBtnClicked(){
  // start timer
  const start_time = new Date();

  // prepare our canvas/ctx
  const canvas = document.getElementById('drawing');
  const ctx = canvas.getContext('2d');
  const yaml_str = document.getElementById('input_yaml').value;

  let {width, height} = await parseYamlForSceneData();
  canvas.width = width;
  canvas.height = height;

  // start the rendering
  toggleWorkInProgress();
  await startParallelRendering(ctx, yaml_str, height);
  toggleWorkInProgress()

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

async function startParallelRendering(ctx, yaml_str, sceneHeight) {
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

  // browsers report untrue cpu-count, so we add some on top.
  const workerCount = getAvailableCores();
  const perWorker = sceneHeight / workerCount;
  let tasks = [];

  for (let i = 0; i < workerCount; i++) {
    tasks.push(spawnRenderer(i * perWorker, (i + 1) * perWorker));
  }

  await Promise.all(tasks);
}

function getAvailableCores() {
  const coresNrInput = document.getElementById('input_cores');
  const coreCount = coresNrInput.value > 0 ? coresNrInput.value : navigator.hardwareConcurrency || 4;
  console.log(`browser reported ${navigator.hardwareConcurrency} cores, multithreading with ${coreCount} concurrent workers.`)
  coresNrInput.value = coreCount;
  return coreCount;
}

//
//              Handle buttons/navigation/ui-feedback
//

function setupInitialCanvas(url) {
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

document.getElementById("sphere_yaml").addEventListener("click", () => {
  changeOpenFile("sphere_yaml");
});

document.getElementById("checkers_yaml").addEventListener("click", () => {
  changeOpenFile("checkers_yaml");
});

document.getElementById("cover_yaml").addEventListener("click", () => {
  changeOpenFile("cover_yaml");
});

// use this global variable to store 'state'
var activeFile = "cover_yaml"

async function changeOpenFile(filename) {
  document.getElementById(activeFile).classList.remove("fileopen");
  document.getElementById(filename).classList.add("fileopen");
  await loadTextareaFromFile("./" + filename.replace("_", "."))
  activeFile = filename;
  if (document.getElementById('checkbox_onload').checked ==true) {
    handleRenderBtnClicked();
  }
}

async function loadTextareaFromFile(path) {
  await fetch(path)
    .then(response => response.text())
    .then((text) => {
      document.getElementById("input_yaml").value = text;
    });
}

// disable multiple renders at once by using this global to keep track of 'state'
var state = "ready";
function toggleWorkInProgress() {
  if (state === 'ready') {
    const btn = document.getElementById("render");
    btn.disabled = true;
    btn.classList.add("disabled");
    const box = document.getElementById("checkbox_onload");
    box.disabled = true;
    state = box.checked ? "wasChecked" : "notChecked"
    box.checked = false;
  } else {
    const btn = document.getElementById("render");
    btn.disabled = false;
    btn.classList.remove("disabled");
    const box = document.getElementById("checkbox_onload");
    box.disabled = false;
    box.checked = state === "wasChecked" ? true : false;
    state = "ready";
  }
}