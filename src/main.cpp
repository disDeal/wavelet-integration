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

void wavelet(double* arr1, double* arr2, int N) {
  auto off = N / 2;
  double* p1 = arr1;
  double* p2 = arr2;
  double* p3;

  for (auto i = 0; i < N; i++) {
    int k1 = i % 2;
    int k = k1 * (-2) + 1;
    arr2[i / 2 + off * k1] = arr1[i / 2] + arr1[i / 2 + 1] * (float)k;
  }

  while (off > 0) {
    p3 = p1;
    p1 = p2;
    p2 = p3;
    off /= 2;
    for (auto i = 0; i < N; i++) {
      int k1 = i % 2;
      int k = k1 * (-2) + 1;
      p2[i / 2 + off * k1] = p1[i / 2] + p1[i / 2 + 1] * (float)k;
    }
  }
}

void print(double* arr, int N) {
  for (size_t i = 0; i < N; i++) {
    std::cout << arr[i] << ' ';
  }
  std::cout << std::endl;
}

int main() {
  int N = 8;
  // std::vector<double> numbers;
  // std::generate_n(std::back_inserter(numbers), N,
  //                 randomNumberBetween(0., 10.));
  double numbers[] = {1., 2., 3., 4., 5., 6., 7., 8.};
  double wave[N];

  print(numbers, N);

  wavelet(numbers, wave, N);

  print(wave, N);

  return 0;
}
