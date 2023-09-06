#ifndef ODEIR_COMMONG_HPP
#define ODEIR_COMMONG_HPP

#include <optional>
#include <memory>
#include <odeir_internal.hpp>

class ModelWrapper;

using MaybeModel = std::optional<std::shared_ptr<ModelWrapper>>;

class ModelWrapper {
protected:
    internal_api::Model *model;

public:
    ModelWrapper()
    : model(internal_api::odeir_new_model()) {}

    ModelWrapper(internal_api::Model *model)
    : model(model) {}

    static MaybeModel tryFromFile(std::string file_path) {
        auto model = internal_api::odeir_json_file_path_to_model(file_path.c_str());
        if (model) {
            return std::make_shared<ModelWrapper>(model);
        }
        else {
            return std::nullopt;
        }
    }

    ~ModelWrapper() {
        internal_api::odeir_free_model(model);
    }

    internal_api::Model *get() const {
        return model;
    }
};

#endif