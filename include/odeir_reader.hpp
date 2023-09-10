#ifndef ODEIR_READER_HPP
#define ODEIR_READER_HPP

#include <optional>
#include <memory>
#include <odeir_internal.hpp>
#include <odeir_common.hpp>

// ---

#include <bits/iterator_concepts.h>
#include <iterator>
#include <iostream>
#include <stdexcept>
#include <vector>
#include <variant>

#include <odeir_iterator.hpp>

namespace reader_api {

class Population {
protected:
    std::shared_ptr<PopulationLinksWrapper> links;

public:
    Population() { std::runtime_error("Can't call reader_api::Population's default constructor"); }

    Population(const Population& other) = delete;

    Population(Population&& other) = default;

    Population(internal_api::Node *nodePtr, std::shared_ptr<ModelWrapper> model) {
        links = std::make_shared<PopulationLinksWrapper>(nodePtr);
    }
};

class Combinator {
protected:
    std::shared_ptr<CombinatorInputsWrapper> inputs;

public:
    Combinator() { std::runtime_error("Can't call reader_api::Combinator's default constructor"); }

    Combinator(const Combinator& other) = delete;

    Combinator(Combinator&& other) = default;

    Combinator(internal_api::Node *nodePtr, std::shared_ptr<ModelWrapper> model) {
        inputs = std::make_shared<CombinatorInputsWrapper>(nodePtr);
    }
};

class Node {
public:
    internal_api::NodeId id;
    std::variant<Population, Combinator> discriminated;
    std::string name;

    // Debug copy and move constructors
    Node(const Node& other) = delete;

    Node(Node&& other) = delete;

    Node(internal_api::NodeId id, internal_api::Node *nodePtr, std::shared_ptr<ModelWrapper> model)
    : id(id)
    {
        internal_api::NodeType nodeType;
        const char *rawName = internal_api::odeir_node_get_info(nodePtr, &nodeType);

        name = std::string(rawName);

        internal_api::odeir_free_cstr(rawName);

        if (nodeType == internal_api::NodeType::Population) {
            discriminated.emplace<Population>(std::move(Population(nodePtr, model)));
        }
        else if (nodeType == internal_api::NodeType::Combinator) {
            discriminated.emplace<Combinator>(std::move(Combinator(nodePtr, model)));
        }
        else {
            throw std::runtime_error("Unknown node type");
        }
    }
};

}

// -----------------------------------------------------------------------------

class NodeIterator : public IterableBase<NodeIterator, internal_api::NodeId, reader_api::Node> {
public:
    using IterableBase::IterableBase;

    NodeIterator(std::shared_ptr<ModelWrapper> model, std::vector<internal_api::NodeId> data)
    : IterableBase(data), model(std::move(model)) {}

private:
    reader_api::Node dereference(size_t index) const override {
        internal_api::NodeId id = data_[index];
        internal_api::Node* nodePtr = internal_api::odeir_model_get_node(model->get(), id);

        return {id, nodePtr, model};
    }

protected:
    std::shared_ptr<ModelWrapper> model;
};

// -----------------------------------------------------------------------------

class ModelWrecker;

using MaybeModelWrecker = std::optional<ModelWrecker>;

class ModelWrecker {

protected:
    std::shared_ptr<ModelWrapper> model;

public:

    explicit ModelWrecker(std::shared_ptr<ModelWrapper> model)
    : model(std::move(model)) {}

    static MaybeModelWrecker tryFrom(std::string path) {
        if (auto modelWrapped = ModelWrapper::tryFromFile(path)) {
            return ModelWrecker(modelWrapped.value());
        }
        else {
            return std::nullopt;
        }
    }

    NodeIterator nodes() {
        size_t len, cap;
        auto nodeIds = internal_api::odeir_model_get_node_ids(model->get(), &len, &cap);

        std::cout << "DBG: " << nodeIds << ", " << len << ", " << cap << std::endl;

        std::vector<internal_api::NodeId> nodeIdsVec(nodeIds, nodeIds + len);

        return {model, nodeIdsVec};
    }

};

#endif