#pragma once

#include "../JuceLibraryCode/JuceHeader.h"

#include "freeverb.hpp"

#include <boost/lockfree/spsc_queue.hpp>
#include <memory>

enum class Parameter {
  Dampening,
  Width,
  RoomSize,
  Freeze,
  Dry,
  Wet,
};

class MainComponent : public AudioAppComponent,
                      public Slider::Listener,
                      public Button::Listener {
public:
  //==============================================================================
  MainComponent();
  ~MainComponent();

  //==============================================================================
  void prepareToPlay(int samplesPerBlockExpected, double sampleRate) override;
  void getNextAudioBlock(const AudioSourceChannelInfo &bufferToFill) override;
  void releaseResources() override;

  //==============================================================================
  void paint(Graphics &g) override;
  void resized() override;

  //==============================================================================
  void sliderValueChanged(Slider *slider) override;

  //==============================================================================
  void buttonClicked(Button *button) override;
  void buttonStateChanged(Button *button) override {}

private:
  std::unique_ptr<freeverb::Freeverb, decltype(&freeverb::destroy)> freeverb_;
  boost::lockfree::spsc_queue<std::pair<Parameter, float>,
                              boost::lockfree::capacity<128>>
      command_queue_;

  Slider sliderDampening_;
  Slider sliderWidth_;
  Slider sliderRoomSize_;
  Slider sliderDry_;
  Slider sliderWet_;

  Label labelDampening_;
  Label labelWidth_;
  Label labelRoomSize_;
  Label labelDry_;
  Label labelWet_;

  ToggleButton toggleFreeze_;

  JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(MainComponent)
};
