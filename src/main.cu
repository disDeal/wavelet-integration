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

__global__ void wavelet(float* a, float* b, int N) {
  __shared__ float arr1[1024];
  __shared__ float arr1[1024];
  float* p1 = arr1;
  float* p2 = arr2;
  float* p3;

  int idx = threadIdx.x;
  arr1[i] = a[i]

  __syncthreads();

  int off = N / 2;

  int k1 = i % 2;
  int k = k1 * (-2) + 1;
  arr2[i / 2 + off * k1] = arr1[i / 2] + arr1[i / 2 + 1] * (float)k;

  while (off > 0) {
    p3 = p1;
    p1 = p2;
    p2 = p3;
    off /= 2;
    p2[i / 2 + off * k1] = p1[i / 2] + p1[i / 2 + 1] * (float)k;
    __syncthreads();
  }
  b[i] = p2[i];
}

void print(float* arr, int N) {
  for (size_t i = 0; i < N; i++) {
    std::cout << arr[i] << ' ';
  }
  std::cout << std::endl;
}

int main() {
  int N = 8;
  // std::vector<float> numbers;
  // std::generate_n(std::back_inserter(numbers), N,
  //                 randomNumberBetween(0., 10.));
  float numbers[] = {1., 2., 3., 4., 5., 6., 7., 8.};
  float wave[N];

  print(numbers, N);

  wavelet(numbers, wave, N);

  print(wave, N);

  return 0;
}
