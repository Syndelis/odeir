#ifndef OPAQUE_RUST_TYPES_HPP
#define OPAQUE_RUST_TYPES_HPP

template <typename T> using Box = T*;

template <typename T> struct Vec;

template <typename K, typename V> struct HashMap;

struct String;

#endif