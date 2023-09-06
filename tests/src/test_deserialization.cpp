#include "odeir_reader.hpp"
#include <odeir.hpp>

#include <catch_amalgamated.hpp>

using Catch::Matchers::Equals;

TEST_CASE( "trying out the API" ) {

    std::cout << "INITIALIZING ASDFG" << std::endl;

    auto maybeModel = ModelWrecker::tryFrom("/home/brenno/Documents/UFSJ/ode-designer/lib/odeir/tests/fixtures/simple.json");

    if (auto model = maybeModel) {
        for (auto node : model->nodes()) {
            std::cout << node << std::endl;
        }
    }

    REQUIRE( false );

}