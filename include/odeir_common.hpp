#ifndef ODEIR_COMMONG_HPP
#define ODEIR_COMMONG_HPP

#include <optional>
#include <memory>
#include <odeir_internal.hpp>
#include <stdexcept>
#include <iostream>

class ModelWrapper;

using MaybeModel = std::optional<std::shared_ptr<ModelWrapper>>;

class ModelWrapper {
protected:
    internal_api::Model *modelPtr;

public:
    static MaybeModel tryFromFile(std::string file_path) {
        auto modelPtr = internal_api::odeir_json_file_path_to_model(file_path.c_str());
        if (modelPtr) {
            return std::make_optional(std::make_shared<ModelWrapper>(modelPtr));
        }
        else {
            return std::nullopt;
        }
    }

    ModelWrapper(internal_api::Model *modelPtr)
    : modelPtr(modelPtr) {}

    ModelWrapper() 
    : modelPtr(internal_api::odeir_new_model()) {}

    ~ModelWrapper() {
        internal_api::odeir_free_model(modelPtr);
    }

    internal_api::Model *get() const {
        return modelPtr;
    }
};

#endif
