#include <catch_amalgamated.hpp>
#include <odeir.hpp>

using Catch::Matchers::Equals;

TEST_CASE( "BoxedSlice Semantics" ) {
    auto empty_slice = BoxedSlice<int>();

    std::vector<int> v = {1, 2, 3, 4, 5};
    auto slice = BoxedSlice<int>(v);
    for(int i = 0; i < slice.length(); i++) {
            REQUIRE(v[i] == slice[i]);
    }
}
