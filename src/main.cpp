#include "rusty_code/src/lib.rs.h"
#include <iostream>

extern "C" {
int32_t rusty_extern_c_integer();
}

int main() {
  std::cout << "A value given via generated cxxbridge "
            << rusty_cxxbridge_integer() << "\n";
  std::cout << "A value given directly by extern c function "
            << rusty_extern_c_integer() << "\n";

  auto blob = rusty_get_blob();
  std::cout << "Blob size: " << blob.size << "\n";
  for (auto tag : blob.tags) {
      std::cout << tag.c_str() << "\n";
  }

  return 0;
}
