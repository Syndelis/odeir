#include "odeir2.hpp"
#include <catch_amalgamated.hpp>
#include <odeir.hpp>
#include <fstream>

#include <iostream>

using Catch::Matchers::Equals;

TEST_CASE( "a json should be deserializable into a Model" ) {

    // Given - a json string, stored in simple_json, auto-generated from fixtures/simple.json

    auto model = new_model(0, 10.50, 0.1);
    auto node1 = new_node_population(1, "Population 1", "Population 1_0");
    node1->add_link('+', 30);
    auto node2 = new_node_population(1, "Population 2", "Population 2_0");
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
    
    auto model_json = model_into_json(std::move(model));

    #include <fixtures/simple.h>
    std::ofstream("/tmp/a.json") << std::string(model_json);

    // When - the json is serialized into a CModel
    /* rust::Slice<const uint8_t> json_slice(fixtures_simple_json, fixtures_simple_json_len);
    auto _model = model_from_json(json_slice);
    const Model& model = *_model;


    // Then - the model should have the correct value()s

    // MetaData --------------------------------------------
    
    REQUIRE( model.meta_data().start_time() == 0 );
    REQUIRE( model.meta_data().end_time() == 10.5 );
    REQUIRE( model.meta_data().delta_time() == 0.1 );

    // Nodes -----------------------------------------------
    
    // REQUIRE( model.nodes.length() == 3 );

    // Node 0 ----------------------------------------------

    // REQUIRE( model.nodes[0].tag == CNode::Tag::Population );
    // REQUIRE( model.nodes[0].population.id == 1 );
    // REQUIRE_THAT( model.nodes[0].population.name(), Equals("Population 1") );
    // REQUIRE_THAT( model.nodes[0].population.related_constant_name(), Equals("Population 1_0") );

    // Node 1 ----------------------------------------------

    // REQUIRE( model.nodes[1].tag == CNode::Tag::Population );
    // REQUIRE( model.nodes[1].population.id == 2 );
    // REQUIRE_THAT( model.nodes[1].population.name(), Equals("Population 2") );
    // REQUIRE_THAT( model.nodes[1].population.related_constant_name(), Equals("Population 2_0") );

    // Node 2 ----------------------------------------------

    // REQUIRE( model.nodes[2].tag == CNode::Tag::Combinator );
    // REQUIRE( model.nodes[2].combinator.id == 30 );
    // REQUIRE_THAT( model.nodes[2].combinator.name(), Equals("Pop1 + Pop2") );
    // REQUIRE( model.nodes[2].combinator.operation == '+' );

    // REQUIRE( model.nodes[2].combinator.inputs.length() == 2 );
    // REQUIRE( model.nodes[2].combinator.inputs[0] == 1 );
    // REQUIRE( model.nodes[2].combinator.inputs[1] == 2 );

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
 */
}
