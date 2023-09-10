#include "odeir_reader.hpp"
#include <odeir.hpp>

#include <catch_amalgamated.hpp>
#include <variant>

using Catch::Matchers::Equals;

TEST_CASE( "trying out the API" ) {

    std::cout << "INITIALIZING ASDFG" << std::endl;

    auto maybeModel = ModelWrecker::tryFrom("/home/brenno/Documents/UFSJ/ode-designer/lib/odeir/tests/fixtures/simple.json");

    if (auto model = maybeModel) {
        for (auto node : model->nodes()) {
            std::cout << node.name << std::endl;

            if (auto *pop = std::get_if<reader_api::Population>(&node.discriminated)) {
                std::cout << "\tIs a population with links: ";

                for (auto link : pop->links) {
                    printf("%c%d, ", link.sign, link.node_id);
                }
            }
            else if (auto *cmb = std::get_if<reader_api::Combinator>(&node.discriminated)) {
                std::cout << "\tIs a combinator with inputs: ";

                for (auto input : cmb->inputs) {
                    std::cout << input << ", ";
                }
            }
            std::cout << std::endl;
        }
    }

    REQUIRE( false );

}