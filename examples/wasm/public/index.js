const audioContext = new AudioContext();

(async () => {
  const response = await fetch('./freeverb_wasm.wasm');
  const wasmBuffer = await response.arrayBuffer();

  await audioContext.audioWorklet.addModule('./freeverb-processor.js');

  navigator.mediaDevices.getUserMedia({ audio: true, video: false })
    .then(function(stream) {
      const mic = audioContext.createMediaStreamSource(stream);

      window.freeverb = new AudioWorkletNode(audioContext, "freeverb-processor", {
        outputChannelCount: [2],
        processorOptions: {
          wasmBuffer,
          sampleRate: audioContext.sampleRate,
        },
      });

      mic.connect(window.freeverb);
      window.freeverb.connect(audioContext.destination);
      audioContext.suspend();
    })
    .catch(function(error) {
      console.log("Failed to initialize audio:", error.name, "-", error.message);
    });
})();

let playing = false;
const play = document.getElementById('play');
play.addEventListener('click', function() {
  if (playing) {
    audioContext.suspend();
    playing = false;
    console.log("Suspending");
  } else {
    audioContext.resume();
    playing = true;
    console.log("Resuming");
  }
});

function setParameter(name, value) {
  window.freeverb.parameters.get(name).setValueAtTime(value, audioContext.currentTime);
}

const dampeningSlider = document.getElementById('dampening');
dampeningSlider.addEventListener('input', function() {
  setParameter('dampening', dampeningSlider.value / 100.0);
});

const widthSlider = document.getElementById('width');
widthSlider.addEventListener('input', function() {
  setParameter('width', widthSlider.value / 100.0);
});

const roomSizeSlider = document.getElementById('room-size');
roomSizeSlider.addEventListener('input', function() {
  setParameter('roomSize', roomSizeSlider.value / 100.0);
});

const drySlider = document.getElementById('dry');
drySlider.addEventListener('input', function() {
  setParameter('dry', drySlider.value / 100.0);
});

const wetSlider = document.getElementById('wet');
wetSlider.addEventListener('input', function() {
  setParameter('wet', wetSlider.value / 100.0);
});

const freezeButton = document.getElementById('freeze');
freezeButton.addEventListener('input', function() {
  setParameter('freeze', freezeButton.checked ? 1 : 0);
});
