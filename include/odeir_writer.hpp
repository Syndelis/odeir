#ifndef ODEIR_WRITER_HPP
#define ODEIR_WRITER_HPP

#include <iostream>
#include <string>
#include <memory>
#include <utility>
#include <type_traits>

#include <odeir_internal.hpp>
#include <odeir_common.hpp>

using internal_api::NodeId;

struct InitialState {};

struct NodeState {
    NodeId id;
    std::string name;
};

struct PopulationState {
    NodeId id;
    std::string name;
    std::string relatedConstant;
};

struct LinkState {
    internal_api::Node *population;
    NodeId id;
    char sign;
};

struct CombinatorState {
    NodeId id;
    std::string name;
    char operation;
};

struct InputState {
    internal_api::Node *combinator;
    NodeId id;
};

struct Complete {};

template<typename CurrentState>
class ModelBuilder {

private:
    void *end() { return nullptr; }

protected:
    std::shared_ptr<ModelWrapper> model;
    CurrentState state;

public:
    explicit ModelBuilder(
        std::shared_ptr<ModelWrapper> model, CurrentState state
    ) : model(std::move(model)),
        state(std::move(state)) {}

    explicit ModelBuilder() :
        model(std::make_shared<ModelWrapper>()),
        state(InitialState{}) {}

    ModelBuilder<NodeState> buildNode(NodeId id, std::string name) {
        static_assert(
            !std::is_same_v<CurrentState, NodeState>,
            "Can't call `buildNode` before finishing to build the current node."
            " Either call `withRelatedConstant` to transmute the current node "
            "to a Population or `withOperation` to transmute it to a "
            "Combinator."
        );
        this->end();
        return ModelBuilder<NodeState>(model, NodeState { .id = id, .name = name });
    }

    ModelBuilder<PopulationState> withRelatedConstant(std::string relatedConstantName);
    ModelBuilder<LinkState> addLink(NodeId id, char sign);

    ModelBuilder<CombinatorState> withOperation(char op);
    ModelBuilder<InputState> addInput(NodeId id);

    ModelBuilder<CurrentState> setMetadata(float startTime, float endTime, float delta) {
        internal_api::odeir_set_metadata(model->get(), startTime, endTime, delta);
        return ModelBuilder<CurrentState>(model, state);
    }

    ModelBuilder<CurrentState> addConstant(std::string name, double value) {
        internal_api::odeir_insert_const(model->get(), name.c_str(), value);
        return ModelBuilder<CurrentState>(model, state);
    }

    std::string toJson();

    void debugPrint() {
        auto str = internal_api::odeir_debug_string_model(model->get());
        std::cout << str << std::endl;
        internal_api::odeir_free_cstr(str);
    }

    operator ModelBuilder<Complete>() {
        this->end();
        return ModelBuilder<Complete>(model, Complete {});
    }

};

using Model = ModelBuilder<Complete>;

#include <odeir.hpp>

template<>
inline void *ModelBuilder<PopulationState>::end() {
    auto node = internal_api::odeir_insert_population(model->get(), state.id, state.name.c_str(), state.relatedConstant.c_str());
    return static_cast<void *>(node);
}

template<>
inline ModelBuilder<PopulationState> ModelBuilder<NodeState>::withRelatedConstant(std::string relatedConstantName) {
    return ModelBuilder<PopulationState>(model, PopulationState { .id = state.id, .name = state.name, .relatedConstant = relatedConstantName });
}

template<>
inline ModelBuilder<LinkState> ModelBuilder<PopulationState>::addLink(NodeId id, char sign) {
    auto node = static_cast<internal_api::Node *>(this->end());
    return ModelBuilder<LinkState>(
        model, LinkState { .population = node, .id = id, .sign = sign }
    );
}

template<>
inline void *ModelBuilder<LinkState>::end() {
    internal_api::odeir_insert_population_link(state.population, state.sign, state.id);
    return nullptr;
}

template<>
inline ModelBuilder<LinkState> ModelBuilder<LinkState>::addLink(NodeId id, char sign) {
    this->end();
    return ModelBuilder<LinkState>(
        model, LinkState { .population = state.population, .id = id, .sign = sign }
    );
}

template<>
inline ModelBuilder<CombinatorState> ModelBuilder<NodeState>::withOperation(char op) {
    return ModelBuilder<CombinatorState>(
        model, CombinatorState {
            .id = state.id, .name = state.name, .operation = op
        }
    );
}

template<>
inline void *ModelBuilder<InputState>::end() {
    internal_api::odeir_insert_combinator_input(state.combinator, state.id);
    return nullptr;
}

template<>
inline void *ModelBuilder<CombinatorState>::end() {
    auto node = internal_api::odeir_insert_combinator(model->get(), state.id, state.name.c_str(), state.operation);
    return static_cast<void *>(node);
}

template<>
inline ModelBuilder<InputState> ModelBuilder<CombinatorState>::addInput(NodeId id) {
    auto node = static_cast<internal_api::Node *>(this->end());
    return ModelBuilder<InputState>(
        model, InputState { .combinator = node, .id = id }
    );
}

template<>
inline ModelBuilder<InputState> ModelBuilder<InputState>::addInput(NodeId id) {
    this->end();
    return ModelBuilder<InputState>(
        model, InputState { .combinator = state.combinator, .id = id }
    );
}

template<>
inline std::string ModelBuilder<Complete>::toJson() {
    auto str = internal_api::odeir_model_to_json(model->get());
    std::string ret(str);
    internal_api::odeir_free_cstr(str);
    return ret;
}


#endif
