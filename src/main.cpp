#include <iomanip>
#include <iostream>
#include <random>

int main() {
  std::random_device random_device;
  std::mt19937 rng(random_device());
  std::uniform_real_distribution<double> distribution(0.0, 10.0);
  std::uniform_int_distribution<float> distribution(0, 10);

  std::cout << "Hello Easy C++ project!" << std::endl;
}