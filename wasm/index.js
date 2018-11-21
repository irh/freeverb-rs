const wasm = import("./freeverb_wasm");

const context = new AudioContext();

wasm.then(wasm => {
  window.freeverb = new wasm.Freeverb();

  navigator.mediaDevices.getUserMedia({audio : true, video : false})
      .then(function(stream) {
        const mic = context.createMediaStreamSource(stream);
        const freeverbProcessor = context.createScriptProcessor(512, 2, 2);

        freeverbProcessor.onaudioprocess = function(event) {
          const input = event.inputBuffer;
          const output = event.outputBuffer;

          window.freeverb.process(
              input.getChannelData(0), input.getChannelData(1),
              output.getChannelData(0), output.getChannelData(1));
        }

        mic.connect(freeverbProcessor);
        freeverbProcessor.connect(context.destination);
        context.suspend();
      })
      .catch(function(error) {
        console.log("Unable to access microphone");
        console.log(error);
      });
});

let playing = false;
const play = document.getElementById('play');
play.addEventListener('click', function() {
  if (playing) {
    context.suspend();
  } else {
    context.resume();
  }
  playing = !playing;
});

const dampeningSlider = document.getElementById('dampening');
dampeningSlider.addEventListener('input', function() {
  window.freeverb.set_dampening(dampeningSlider.value / 100.0);
});

const widthSlider = document.getElementById('width');
widthSlider.addEventListener('input', function() {
  window.freeverb.set_width(widthSlider.value / 100.0);
});

const roomSizeSlider = document.getElementById('room-size');
roomSizeSlider.addEventListener('input', function() {
  window.freeverb.set_room_size(roomSizeSlider.value / 100.0);
});

const drySlider = document.getElementById('dry');
drySlider.addEventListener('input', function() {
  window.freeverb.set_dry(drySlider.value / 100.0);
});

const wetSlider = document.getElementById('wet');
wetSlider.addEventListener('input', function() {
  window.freeverb.set_wet(wetSlider.value / 100.0);
});

const freezeButton = document.getElementById('freeze');
freezeButton.addEventListener('input', function() {
  window.freeverb.set_freeze(freezeButton.checked);
});

