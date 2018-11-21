#include "freeverb.hpp"

#include <memory>

int main() {
  auto pFreeverb = freeverb::create(44100);
  freeverb::set_wet(pFreeverb, 1.0);
  freeverb::destroy(pFreeverb);
}
