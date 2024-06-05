import * as ComLink from "comlink";

// web worker used to parellalize the wasm workload.
class RendererWorker{
  // sets up wasm for this worker and current state.
  // Web workers cant really share a wasm-instance. So they all start their own here. Only the
  // Wasm-Object: 'Renderer' need's to be saved by reference to call into this wasm instance
  async init({start: start, end: end, yaml_str: yaml_str}) {
    await import("../pkg").then((wasm) => {
      wasm.main_js(); // initialize rust-panic -> console.error pipline so errors are passed down.
 
    this.wasmRenderer = new wasm.WasmRenderer(yaml_str);
      this.end = end;
      this.y = start;
    });
  }

  async renderNext() {
    if (this.y >= this.end) {
      return false;
    }

    const imgData = this.wasmRenderer.row_to_image_pixels(this.y);
    return { y: this.y++, imgData: imgData };
  }

  async getWidthHeight() {
    return { width: this.wasmRenderer.width, height: this.wasmRenderer.height }
  }
}

ComLink.expose(new RendererWorker());