#include <odeir_internal.hpp>
#include <catch_amalgamated.hpp>

#include <iostream>

using namespace internal_api;

TEST_CASE( "A model can be serialized using the raw C API" ) {

    // Given - We've built a model from the ground up using the C API

    auto model = odeir_new_model();

    odeir_set_metadata(model, 0.0, 10.5, 0.125);

    odeir_insert_const(model, "gravity", 9.81);
    odeir_insert_const(model, "Population 1_0", 100.0);
    odeir_insert_const(model, "Population 2_0", 200.0);
    odeir_insert_const(model, "a", 1.6);

    auto pop1 = odeir_insert_population(model, 1, "Population 1", "Population 1_0");
    odeir_insert_population_link(pop1, '+', 30);

    auto pop2 = odeir_insert_population(model, 2, "Population 2", "Population 2_0");
    odeir_insert_population_link(pop2, '-', 30);

    auto comb30 = odeir_insert_combinator(model, 30, "Pop1 + Pop2", '+');
    odeir_insert_combinator_input(comb30, 1);
    odeir_insert_combinator_input(comb30, 2);

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

TEST_CASE( "A JSON can be deserialized using the raw C API" ) {

    // Given - A JSON representing a model

    #include <fixtures/simple.h>

    // When - I deserialize the JSON into a model

    auto model = odeir_json_to_model((const char *) fixtures_simple_json);

    // Then - The deserialized model should be equal to the expected model

    REQUIRE ( model != nullptr );

    // Nodes -------------------------------------------------------------------

    size_t len, cap;
    auto node_ids = odeir_model_get_node_ids(model, &len, &cap);

    std::vector node_ids_vec(node_ids, node_ids + len);

    REQUIRE_THAT(
        node_ids_vec,
        Catch::Matchers::UnorderedEquals(std::vector<NodeId>{ 30, 2, 1 })
    );

    odeir_free_node_id_vec(node_ids, len, cap);

    // Node 1 ------------------------------------------------------------------

    auto node = odeir_model_get_node(model, 1);

    REQUIRE ( node != nullptr );

    NodeType node_type;
    auto node_name = odeir_node_get_info(node, &node_type);

    REQUIRE ( node_type == NodeType::Population );
    REQUIRE_THAT ( node_name, Catch::Matchers::Equals("Population 1") );

    auto node_links = odeir_population_take_links(node, &len, &cap);

    std::vector node_1_links(node_links, node_links + len);

    REQUIRE_THAT(
        node_1_links,
        Catch::Matchers::UnorderedEquals(std::vector<Link>{ Link{ '+', 30 } })
    );

    odeir_free_link_vec(node_links, len, cap);

    odeir_free_cstr(node_name);

    // Node 2 ------------------------------------------------------------------

    node = odeir_model_get_node(model, 2);

    REQUIRE ( node != nullptr );

    node_name = odeir_node_get_info(node, &node_type);

    REQUIRE ( node_type == NodeType::Population );
    REQUIRE_THAT ( node_name, Catch::Matchers::Equals("Population 2") );

    node_links = odeir_population_take_links(node, &len, &cap);

    std::vector node_2_links(node_links, node_links + len);

    REQUIRE_THAT(
        node_2_links,
        Catch::Matchers::UnorderedEquals(std::vector<Link>{ Link{ '-', 30 } })
    );

    odeir_free_link_vec(node_links, len, cap);

    odeir_free_cstr(node_name);

    // Node 30 -----------------------------------------------------------------

    node = odeir_model_get_node(model, 30);

    REQUIRE ( node != nullptr );

    node_name = odeir_node_get_info(node, &node_type);

    REQUIRE ( node_type == NodeType::Combinator );
    REQUIRE_THAT ( node_name, Catch::Matchers::Equals("Pop1 + Pop2") );

    auto node_inputs = odeir_combinator_take_inputs(node, &len, &cap);

    std::vector node_30_inputs(node_inputs, node_inputs + len);

    REQUIRE_THAT(
        node_30_inputs,
        Catch::Matchers::UnorderedEquals(std::vector<NodeId>{ 1, 2 })
    );

    odeir_free_node_id_vec(node_inputs, len, cap);

    odeir_free_cstr(node_name);

    // Constants ---------------------------------------------------------------

    // Clean up ----------------------------------------------------------------

    odeir_free_model(model);

}
