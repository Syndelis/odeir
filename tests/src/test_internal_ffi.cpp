#include <odeir_internal.hpp>
#include <catch_amalgamated.hpp>

#include <iostream>

TEST_CASE( "A model can be serialized using the raw C API" ) {

    // Given - We've built a model from the ground up using the C API

    auto model = odeir_new_model();

    odeir_set_metadata(model, 0.0, 10.5, 0.1);

    odeir_insert_const(model, "gravity", 9.81);
    odeir_insert_const(model, "Population 1_0", 100.0);
    odeir_insert_const(model, "Population 2_0", 200.0);
    odeir_insert_const(model, "a", 1.6);

    odeir_insert_population(model, 1, "Population 1", "Population 1_0");
    odeir_insert_population_link(model, 1, '+', 30);

    odeir_insert_population(model, 2, "Population 2", "Population 2_0");
    odeir_insert_population_link(model, 2, '-', 30);

    odeir_insert_combinator(model, 30, "Pop1 + Pop2", '+');
    odeir_insert_combinator_input(model, 30, 1);
    odeir_insert_combinator_input(model, 30, 2);

    // When - I serialize the model into a JSON

    auto model_str = odeir_model_to_json(model);

    // Then - It should be equal to the expected JSON

    #include <fixtures/simple.h>

    REQUIRE (
        odeir_debug_compare_jsons(
            (const char *) model_str,
            (const char *) fixtures_simple_json
        )
    );

    // Clean up

    odeir_free_cstr(model_str);

    odeir_free_model(model);

}
