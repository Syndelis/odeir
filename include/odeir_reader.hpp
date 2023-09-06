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

template <typename Derived, typename SourceType, typename ReturnType>
class IterableBase {
public:
    IterableBase(const std::vector<SourceType>& data) : data_(data) {}

    class Iterator {
    public:
        explicit Iterator(const IterableBase* iterable, size_t index) : iterable_(iterable), index_(index) {}

        ReturnType operator*() {
            return iterable_->dereference(index_);
        }

        Iterator& operator++() {
            ++index_;
            return *this;
        }

        bool operator!=(const Iterator& other) const {
            return index_ != other.index_;
        }

    private:
        const IterableBase* iterable_;
        size_t index_;
    };

    Iterator begin() const {
        return Iterator(static_cast<const Derived*>(this), 0);
    }

    Iterator end() const {
        return Iterator(static_cast<const Derived*>(this), data_.size());
    }

private:
    virtual ReturnType dereference(size_t index) const = 0;

protected:
    std::vector<SourceType> data_;
};

// -----------------------------------------------------------------------------

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