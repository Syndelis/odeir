#include "odeir_internal.hpp"
#include <odeir.hpp>
#include <catch_amalgamated.hpp>

using Catch::Matchers::Equals;
using namespace odeir;

TEST_CASE( "a Model should be serializable into a JSON" ) {

    // Given - A model created from C++ code

    Model model = ModelBuilder<InitialState>()
        .setMetadata(0, 10.5, 0.125)
        .buildNode(1, "Population 1")
            .withInitialPopulation(100)
            .addLink(30, LinkType::Normal)
        .buildNode(2, "Population 2")
            .withInitialPopulation(200)
            .addLink(30, LinkType::Negative)
        .buildNode(30, "Pop1 + Pop2")
            .withOperation('+')
            .addInput(1)
            .addInput(2)
        .buildNode(3, "gravity")
            .withConstant(9.81)
        .buildNode(4, "a")
            .withConstant(1.6);

    // When - The model is serialized into a json
    
    auto model_json = model.toJson();

    // Then - The resulting JSON should be valid

    REQUIRE( !model_json.empty() );

    #include <fixtures/simple.h>

    REQUIRE( internal_api::odeir_debug_compare_jsons(
        model_json.c_str(), (const char *) fixtures_simple_json
    ) );

}
