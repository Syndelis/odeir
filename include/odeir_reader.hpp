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
#include <vector>

#include <odeir_iterator.hpp>

class NodeIterator : public IterableBase<NodeIterator, internal_api::NodeId, internal_api::Node*> {
public:
    using IterableBase::IterableBase;

    NodeIterator(std::shared_ptr<ModelWrapper> model, std::vector<internal_api::NodeId> data)
    : IterableBase(data), model(std::move(model)) {}

private:
    internal_api::Node* dereference(size_t index) const override {
        internal_api::NodeId id = data_[index];
        return internal_api::odeir_model_get_node(model->get(), id);
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