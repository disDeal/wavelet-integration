#include <algorithm>
#include <iostream>
#include <random>
#include <vector>

auto randomNumberBetween = [](int low, int high) {
  auto randomFunc =
      [distribution_ = std::uniform_int_distribution<int>(low, high),
       random_engine_ = std::mt19937{std::random_device{}()}]() mutable {
        return distribution_(random_engine_);
      };
  return randomFunc;
};

std::vector<double> addition(std::vector<double> arr) {
  size_t size = arr.size();
  for (size_t i = 0; i < size; i++) {
    arr[i] += 1;
  }

  return arr;
}

int main() {
  // std::vector<double> numbers;
  // std::generate_n(std::back_inserter(numbers), 10,
  //                 randomNumberBetween(0., 10.));
  std::vector<double> numbers = {1., 2., 3., 4., 5., 6., 7., 8., 9., 10.};

  for (auto number : addition(numbers)) {
    std::cout << number << ' ';
  }
}

template <typename T>
void print(std::vector<T> const& input) {
  std::copy(input.begin(), input.end(),
            std::experimental::make_ostream_joiner(std::cout, " "));
}
