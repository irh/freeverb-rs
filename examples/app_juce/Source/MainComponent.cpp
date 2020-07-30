#include "MainComponent.h"

//==============================================================================
MainComponent::MainComponent() : freeverb_(nullptr, &freeverb::destroy) {
  auto setupSlider = [this](auto &slider, auto &label, const auto &text,
                            const auto value) {
    addAndMakeVisible(slider);
    addAndMakeVisible(label);

    slider.setRange(0, 100);
    slider.addListener(this);
    slider.setTextValueSuffix("%");
    slider.setNumDecimalPlacesToDisplay(1);
    slider.setValue(value);

    label.setText(text, dontSendNotification);
    label.attachToComponent(&slider, true);
  };

  setupSlider(sliderDampening_, labelDampening_, "Dampening", 40.0);
  setupSlider(sliderWidth_, labelWidth_, "Width", 30.0);
  setupSlider(sliderRoomSize_, labelRoomSize_, "Room Size", 60.0);
  setupSlider(sliderDry_, labelDry_, "Dry", 50.0);
  setupSlider(sliderWet_, labelWet_, "Wet", 30.0);

  addAndMakeVisible(toggleFreeze_);
  toggleFreeze_.addListener(this);
  toggleFreeze_.setButtonText("Freeze");

  setSize(600, 300);
  setAudioChannels(2, 2);
}

MainComponent::~MainComponent() { shutdownAudio(); }

//==============================================================================
void MainComponent::prepareToPlay(int samplesPerBlockExpected,
                                  double sampleRate) {
  freeverb_.reset(freeverb::create(sampleRate));
}

void MainComponent::getNextAudioBlock(const AudioSourceChannelInfo &block) {
  const auto offset = block.startSample;
  const auto buffer = block.buffer;

  std::pair<Parameter, float> command;

  while (command_queue_.pop(command)) {
    switch (command.first) {
    case Parameter::Dampening:
      freeverb::set_dampening(freeverb_.get(), command.second);
      break;
    case Parameter::Width:
      freeverb::set_width(freeverb_.get(), command.second);
      break;
    case Parameter::RoomSize:
      freeverb::set_room_size(freeverb_.get(), command.second);
      break;
    case Parameter::Freeze:
      freeverb::set_freeze(freeverb_.get(), command.second != 0.0);
      break;
    case Parameter::Dry:
      freeverb::set_dry(freeverb_.get(), command.second);
      break;
    case Parameter::Wet:
      freeverb::set_wet(freeverb_.get(), command.second);
      break;
    }
  }

  freeverb::process(freeverb_.get(), buffer->getReadPointer(0, offset),
                    buffer->getReadPointer(1, offset),
                    buffer->getWritePointer(0, offset),
                    buffer->getWritePointer(1, offset), block.numSamples);
}

void MainComponent::releaseResources() { freeverb_.release(); }

//==============================================================================
void MainComponent::paint(Graphics &g) {
  g.fillAll(getLookAndFeel().findColour(ResizableWindow::backgroundColourId));
}

void MainComponent::resized() {
  const auto marginLeft = 120;
  const auto marginRight = 60;
  const auto height = 20;
  const auto verticalPadding = 5;

  auto y = verticalPadding;

  sliderDampening_.setBounds(marginLeft, y,
                             getWidth() - marginLeft - marginRight, height);
  y += height + verticalPadding;

  sliderWidth_.setBounds(marginLeft, y, getWidth() - marginLeft - marginRight,
                         height);
  y += height + verticalPadding;

  sliderRoomSize_.setBounds(marginLeft, y,
                            getWidth() - marginLeft - marginRight, height);
  y += height + verticalPadding;

  toggleFreeze_.setBounds(marginLeft, y, 200, height);
  y += height + verticalPadding;

  sliderDry_.setBounds(marginLeft, y, getWidth() - marginLeft - marginRight,
                       height);
  y += height + verticalPadding;

  sliderWet_.setBounds(marginLeft, y, getWidth() - marginLeft - marginRight,
                       height);
  y += height + verticalPadding;

}

void MainComponent::buttonClicked(Button *button) {
  if (button == &toggleFreeze_) {
    command_queue_.push(
        std::make_pair(Parameter::Freeze, button->getToggleState() ? 1.0 : 0.0));
  }
}

void MainComponent::sliderValueChanged(Slider *slider) {
  if (slider == &sliderDampening_) {
    command_queue_.push(
        std::make_pair(Parameter::Dampening, slider->getValue() / 100.0));
  } else if (slider == &sliderWidth_) {
    command_queue_.push(
        std::make_pair(Parameter::Width, slider->getValue() / 100.0));
  } else if (slider == &sliderRoomSize_) {
    command_queue_.push(
        std::make_pair(Parameter::RoomSize, slider->getValue() / 100.0));
  } else if (slider == &sliderDry_) {
    command_queue_.push(
        std::make_pair(Parameter::Dry, slider->getValue() / 100.0));
  } else if (slider == &sliderWet_) {
    command_queue_.push(
        std::make_pair(Parameter::Wet, slider->getValue() / 100.0));
  }
}
