#include "odeir.hpp"
#include <catch_amalgamated.hpp>
#include <odeir.hpp>

#include <iostream>

using Catch::Matchers::Equals;

TEST_CASE( "a json should be deserializable into a CModel" ) {

    // Given - a json string, stored in simple_json, auto-generated from fixtures/simple.json

    #include <fixtures/simple.h>

    // When - the json is deserialized into a CModel

    rust::Slice<const uint8_t> json_slice(fixtures_simple_json, fixtures_simple_json_len);
    auto _model = model_from_json(json_slice);
    const Model& model = *_model;


    // Then - the model should have the correct value()s

    // MetaData --------------------------------------------
    
    REQUIRE( model.meta_data().start_time() == 0 );
    REQUIRE( model.meta_data().end_time() == 10.5 );
    REQUIRE( model.meta_data().delta_time() == 0.1 );

    // Nodes -----------------------------------------------
    
    // REQUIRE( model.nodes().length() == 3 );

    auto node_ids = new int[10];
    int node_count = model.node_ids(node_ids);

    std::vector node_ids_vec(node_ids, node_ids + node_count);
    free(node_ids);

    REQUIRE( node_ids_vec.size() == 3 );
    REQUIRE_THAT( node_ids_vec, Catch::Matchers::UnorderedEquals(std::vector<int>{ 30, 2, 1 }));

    // Node 1 ----------------------------------------------

    auto node1 = model.get_node(1);

    REQUIRE( node1->tag() == NodeTag::Population );
    REQUIRE( node1->id() == 1 );
    REQUIRE_THAT( node1->name().data(), Equals("Population 1") );
    REQUIRE_THAT( node1->related_constant_name().data(), Equals("Population 1_0") );

    // Node 2 ----------------------------------------------

    auto node2 = model.get_node(2);

    REQUIRE( node2->tag() == NodeTag::Population );
    REQUIRE( node2->id() == 2 );

    // TODO: This is being corrupted. Correctly starts with "Population 2", but ends with one or two garbage characters.
    REQUIRE_THAT( node2->name().data(), Equals("Population 2") );
    REQUIRE_THAT( node2->related_constant_name().data(), Equals("Population 2_0") );

    // Node 30 ----------------------------------------------

    auto node30 = model.get_node(30);

    REQUIRE( node30->tag() == NodeTag::Combinator );
    REQUIRE( node30->id() == 30 );
    REQUIRE_THAT( node30->name().data(), Equals("Pop1 + Pop2") );
    // REQUIRE( node30.operation() == '+' );

    // REQUIRE( node30.inputs.length() == 2 );
    // REQUIRE( node30.inputs[0] == 1 );
    // REQUIRE( node30.inputs[1] == 2 );

    // Constants ------------------------------------------

    REQUIRE( model.constants().length() == 4 );

    // Constant 0 -----------------------------------------

    REQUIRE_THAT( std::string(model.constants()[0].name()), Equals("gravity") );
    REQUIRE( model.constants()[0].value() == 9.81 );

    // Constant 1 -----------------------------------------

    REQUIRE_THAT( std::string(model.constants()[1].name()), Equals("Population 1_0") );
    REQUIRE( model.constants()[1].value() == 100 );

    // Constant 2 -----------------------------------------

    REQUIRE_THAT( std::string(model.constants()[2].name()), Equals("Population 2_0") );
    REQUIRE( model.constants()[2].value() == 200 );

    // Constant 3 -----------------------------------------

    REQUIRE_THAT( std::string(model.constants()[3].name()), Equals("a") );
    REQUIRE( model.constants()[3].value() == 1.6 );

}
