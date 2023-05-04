#include <odeir.hpp>
#include <catch_amalgamated.hpp>
#include <iostream>

using Catch::Matchers::Equals;

TEST_CASE( "a json should be deserializable into a Model" ) {

    // Given - A model created from C++ code

    auto model = new_model(0, 10.50, 0.1);

    auto node1 = new_node_population(1, "Population 1", "Population 1_0");
    node1->add_link('+', 30);

    auto node2 = new_node_population(2, "Population 2", "Population 2_0");
    node2->add_link('-', 30);

    auto node3 = new_node_combinator(30, "Pop1 + Pop2", '+');
    node3->add_input(1);
    node3->add_input(2);

    model->add_node(std::move(node1));
    model->add_node(std::move(node2));
    model->add_node(std::move(node3));

    model->add_constant("gravity", 9.81);
    model->add_constant("Population 1_0", 100);
    model->add_constant("Population 2_0", 200);
    model->add_constant("a", 1.6);

    // When - The model is serialized into a json
    
    auto model_json = model_into_json(std::move(model));

    // Then - The resulting JSON should be valid

    REQUIRE( !model_json.empty() );

}
