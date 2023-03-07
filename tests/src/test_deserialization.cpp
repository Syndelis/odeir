#include <catch_amalgamated.hpp>
#include <odeir.h>

using Catch::Matchers::Equals;

TEST_CASE( "a json should be deserializable into a CModel" ) {

    // Given - a json string, stored in simple_json, auto-generated from fixtures/simple.json

    #include <fixtures/simple.h>

    // When - the json is deserialized into a CModel

    CModel model = model_from_cstring(fixtures_simple_json);

    // Then - the model should have the correct values

    // MetaData --------------------------------------------
    
    REQUIRE( model.meta_data.start_time == 0 );
    REQUIRE( model.meta_data.end_time == 10.5 );
    REQUIRE( model.meta_data.delta_time == 0.1 );

    // Nodes -----------------------------------------------
    
    REQUIRE( model.node_count == 3 );

    // Node 0 ----------------------------------------------

    REQUIRE( model.nodes[0].tag == CNode::Tag::Population );
    REQUIRE( model.nodes[0].population.id == 1 );
    REQUIRE_THAT( model.nodes[0].population.name, Equals("Population 1") );
    REQUIRE_THAT( model.nodes[0].population.related_constant_name, Equals("Population 1_0") );

    // Node 1 ----------------------------------------------

    REQUIRE( model.nodes[1].tag == CNode::Tag::Population );
    REQUIRE( model.nodes[1].population.id == 2 );
    REQUIRE_THAT( model.nodes[1].population.name, Equals("Population 2") );
    REQUIRE_THAT( model.nodes[1].population.related_constant_name, Equals("Population 2_0") );

    // Node 2 ----------------------------------------------

    REQUIRE( model.nodes[2].tag == CNode::Tag::Combinator );
    REQUIRE( model.nodes[2].combinator.id == 30 );
    REQUIRE_THAT( model.nodes[2].combinator.name, Equals("Pop1 + Pop2") );
    REQUIRE( model.nodes[2].combinator.operation == '+' );

    REQUIRE( model.nodes[2].combinator.input_count == 2 );
    REQUIRE( model.nodes[2].combinator.inputs[0] == 1 );
    REQUIRE( model.nodes[2].combinator.inputs[1] == 2 );

    // Constants ------------------------------------------

    REQUIRE( model.constant_count == 4 );

    // Constant 0 -----------------------------------------

    REQUIRE_THAT( model.constants[0].name, Equals("gravity") );
    REQUIRE( model.constants[0].value == 9.81 );

    // Constant 1 -----------------------------------------

    REQUIRE_THAT( model.constants[1].name, Equals("Population 1_0") );
    REQUIRE( model.constants[1].value == 100 );

    // Constant 2 -----------------------------------------

    REQUIRE_THAT( model.constants[2].name, Equals("Population 2_0") );
    REQUIRE( model.constants[2].value == 200 );

    // Constant 3 -----------------------------------------

    REQUIRE_THAT( model.constants[3].name, Equals("a") );
    REQUIRE( model.constants[3].value == 1.6 );

}
