#ifndef ODEIR_HPP
#define ODEIR_HPP

#include <iostream>
#include <string>
#include <memory>
#include <utility>
#include <type_traits>

#include <odeir_internal.hpp>

namespace odeir {

using internal_api::NodeId;
using internal_api::LinkType;
using internal_api::Operation;

class ModelWrapper {
protected:
    internal_api::Model *model;

public:
    ModelWrapper() : model(internal_api::odeir_new_model()) {}
    ~ModelWrapper() {
        odeir_free_model(model);
    }

    internal_api::Model *get() const {
        return model;
    }
};

struct InitialState {};

struct NodeState {
    NodeId id;
    std::string name;
};

struct PopulationState {
    NodeId id;
    std::string name;
    double initialPopulation;
};

struct ConstantState {
    NodeId id;
    std::string name;
};

struct CombinatorState {
    NodeId id;
    std::string name;
    Operation operation;
};

struct AddingOutputState {
    internal_api::Node *node;
    NodeId id;
    LinkType type;
};

struct AddingInputState {
    internal_api::Node *node;
    NodeId id;
    LinkType type;
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
            !std::is_same<CurrentState, NodeState>::value,
            "Can't call `buildNode` before finishing to build the current node."
            " Either call `withInitialPopulation` to transmute the current node "
            "to a Population, `withOperation` to transmute it to a "
            "Combinator or withConstant to transmute it to a Constant."
        );
        this->end();
        return ModelBuilder<NodeState>(model, NodeState { .id = id, .name = name });
    }

    ModelBuilder<PopulationState> withInitialPopulation(double value);
    ModelBuilder<CombinatorState> withOperation(Operation op);

    ModelBuilder<AddingOutputState> addOutput(NodeId id, LinkType type);
    ModelBuilder<AddingInputState> addInput(NodeId id, LinkType type);

    ModelBuilder<CurrentState> setMetadata(float startTime, float endTime, float delta) {
        internal_api::odeir_set_metadata(model->get(), startTime, endTime, delta);
        return ModelBuilder<CurrentState>(model, state);
    }

    ModelBuilder<ConstantState> withConstant(double value);

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

template<>
void *ModelBuilder<PopulationState>::end() {
    auto node = internal_api::odeir_insert_population(model->get(), state.id, state.name.c_str(), state.initialPopulation);
    return static_cast<void *>(node);
}

template<>
ModelBuilder<PopulationState> ModelBuilder<NodeState>::withInitialPopulation(double value) {
    return ModelBuilder<PopulationState>(model, PopulationState { .id = state.id, .name = state.name, .initialPopulation = value });
}

template<>
ModelBuilder<AddingOutputState> ModelBuilder<PopulationState>::addOutput(NodeId id, LinkType type) {
    auto node = static_cast<internal_api::Node *>(this->end());
    return ModelBuilder<AddingOutputState>(
        model, AddingOutputState { .node = node, .id = id, .type = type }
    );
}

template<>
void *ModelBuilder<AddingOutputState>::end() {
    internal_api::odeir_node_insert_link(state.node, state.type, state.id);
    return nullptr;
}

template<>
ModelBuilder<AddingOutputState> ModelBuilder<AddingOutputState>::addOutput(NodeId id, LinkType type) {
    this->end();
    return ModelBuilder<AddingOutputState>(
        model, AddingOutputState { .node = state.node, .id = id, .type = type }
    );
}

template<> ModelBuilder<ConstantState> ModelBuilder<NodeState>::withConstant(double value) {
    internal_api::odeir_insert_const(model->get(), state.id, state.name.c_str(), value);
    return ModelBuilder<ConstantState>(model, ConstantState { .id = state.id, .name = state.name });
}

template<>
ModelBuilder<CombinatorState> ModelBuilder<NodeState>::withOperation(Operation op) {
    return ModelBuilder<CombinatorState>(
        model, CombinatorState {
            .id = state.id, .name = state.name, .operation = op
        }
    );
}

template<>
void *ModelBuilder<AddingInputState>::end() {
    internal_api::odeir_combinator_insert_input(state.node, state.type, state.id);
    return nullptr;
}

template<>
void *ModelBuilder<CombinatorState>::end() {
    auto node = internal_api::odeir_insert_combinator(model->get(), state.id, state.name.c_str(), state.operation);
    return static_cast<void *>(node);
}

template<>
ModelBuilder<AddingInputState> ModelBuilder<CombinatorState>::addInput(NodeId id, LinkType type) {
    auto node = static_cast<internal_api::Node *>(this->end());
    return ModelBuilder<AddingInputState>(
        model, AddingInputState { .node = node, .id = id, .type = type }
    );
}

template<>
ModelBuilder<AddingInputState> ModelBuilder<AddingInputState>::addInput(NodeId id, LinkType type) {
    this->end();
    return ModelBuilder<AddingInputState>(
        model, AddingInputState { .node = state.node, .id = id }
    );
}

template<>
std::string ModelBuilder<Complete>::toJson() {
    auto str = internal_api::odeir_model_to_json(model->get());
    std::string ret(str);
    internal_api::odeir_free_cstr(str);
    return ret;
}

} // namespace odeir

#endif
