class FreeverbProcessor extends AudioWorkletProcessor {
  static get parameterDescriptors() {
    return [
      {
        name: 'dampening',
        defaultValue: 0.5,
        minValue: 0.0,
        maxValue: 1.0,
        automationRate: 'k-rate'
      },
      {
        name: 'width',
        defaultValue: 0.5,
        minValue: 0.0,
        maxValue: 1.0,
        automationRate: 'k-rate'
      },
      {
        name: 'roomSize',
        defaultValue: 0.5,
        minValue: 0.0,
        maxValue: 1.0,
        automationRate: 'k-rate'
      },
      {
        name: 'freeze',
        defaultValue: 0,
        minValue: 0,
        maxValue: 1,
        automationRate: 'k-rate'
      },
      {
        name: 'dry',
        defaultValue: 0.5,
        minValue: 0.0,
        maxValue: 1.0,
        automationRate: 'k-rate'
      },
      {
        name: 'wet',
        defaultValue: 0.5,
        minValue: 0.0,
        maxValue: 1.0,
        automationRate: 'k-rate'
      },
    ];
  }

  constructor(options) {
    super();

    const wasmBuffer = options.processorOptions.wasmBuffer;
    const module = new WebAssembly.Module(options.processorOptions.wasmBuffer);
    this.wasm = new WebAssembly.Instance(module);

    const exports = this.wasm.exports;
    this.freeverb = exports.create(options.processorOptions.sampleRate);
    this.bufferInLPtr = exports.createBuffer(128);
    this.bufferInL = new Float32Array(exports.memory.buffer, this.bufferInLPtr, 128);
    this.bufferInRPtr = exports.createBuffer(128);
    this.bufferInR = new Float32Array(exports.memory.buffer, this.bufferInRPtr, 128);
    this.bufferOutLPtr = exports.createBuffer(128);
    this.bufferOutL = new Float32Array(exports.memory.buffer, this.bufferOutLPtr, 128);
    this.bufferOutRPtr = exports.createBuffer(128);
    this.bufferOutR = new Float32Array(exports.memory.buffer, this.bufferOutRPtr, 128);
  }

  process(inputs, outputs, parameters) {
    let freeverb = this.freeverb;
    if (!freeverb) {
      console.log("Missing freeverb");
      return false;
    }

    const input = inputs[0];
    const output = outputs[0];

    if (input.length == 0 || output.length < 2) {
      console.log("Missing io (inputs: %i, outputs: %i)", input.length, output.length);
      return false;
    }

    const exports = this.wasm.exports;
    exports.set_dampening(freeverb, parameters.dampening[0]);
    exports.set_width(freeverb, parameters.width[0]);
    exports.set_room_size(freeverb, parameters.roomSize[0]);
    exports.set_freeze(freeverb, parameters.freeze[0]);
    exports.set_dry(freeverb, parameters.dry[0]);
    exports.set_wet(freeverb, parameters.wet[0]);

    if (input.length == 1) {
      this.bufferInL.set(input[0]);
      this.bufferInR.set(input[0]);
    } else {
      this.bufferInL.set(input[0]);
      this.bufferInR.set(input[1]);
    }

    exports.process(
      freeverb, this.bufferInLPtr, this.bufferInRPtr, this.bufferOutLPtr, this.bufferOutRPtr, 128);

    output[0].set(this.bufferOutL);
    output[1].set(this.bufferOutR);

    return true;
  }
}

registerProcessor('freeverb-processor', FreeverbProcessor);
