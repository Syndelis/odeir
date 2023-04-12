#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include <stdexcept>
#include <vector>
#include "opaque_rust_types.hpp"


template<typename T>
struct BoxedSlice {
    const T *ptr;
    size_t len;
    int (*destructor)(const BoxedSlice*);
    ~BoxedSlice() {
        if(destructor != nullptr && !destructor(this)) {
            std::printf("Failed to free BoxedSlice %p\n", this);
        }
    }
    BoxedSlice(): ptr(nullptr), len(0), destructor(nullptr) {}
    BoxedSlice(std::vector<T> vector): len(vector.size()) {
        T* mem = new T[vector.size()];
        std::copy(vector.begin(), vector.end(), mem);
        this->destructor = [](const BoxedSlice* slice) {
            delete[] slice->ptr;
            return 1;
        };
        this->ptr = mem;
    }
    size_t length() const {
        return len;
    }
    const T& operator[](size_t idx) const {
        if (idx < len) {
            return ptr[idx];
        } else {
            throw std::out_of_range("Index out of bounds");
        }
    }
};


extern "C" {

BoxedSlice _tmp();

} // extern "C"
