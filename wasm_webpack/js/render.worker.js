import * as ComLink from "comlink";

// code for our web-workers (so we dont block our ui thread AND can parallelize)

ComLink.expose({
  world: undefined,
  range: undefined,
  y: undefined,
  init(size, lineRange) {
    return import("../pkg").then((wasm) => {
      wasm.main_js();
      this.world = new wasm.World(size);
      this.range = lineRange;
      this.y = lineRange.start;
    });
  },
  renderNext() {
    if (this.y >= this.range.end) {
      return false;
    }

    const data = this.world.render(this.y);
    const retVal = { y: this.y++, data };
    // return Comlink.transfer(retVal, [retVal.data.data]);
    return retVal;
  },
});