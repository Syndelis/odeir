#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include <stdexcept>


struct MetaData {
    double start_time;
    double end_time;
    double delta_time;
};

using NodeId = uint32_t;

struct Link {
    uint32_t sign;
    NodeId node_id;
};

template<typename T>
struct BoxedSlice {
    const T *ptr;
    size_t len;
    int (*destructor)(const BoxedSlice*);
    ~BoxedSlice() {
        if(!destructor(this)) {
            std::printf("Failed to free BoxedSlice %p\n", this);
        }
    }
    size_t length() const {
        return len;
    }
    const T& operator[](size_t idx) const {
        if (idx < len) {
            return ptr[idx];
        } else {
            throw std::out_of_range("Index out of bounds");
        }
    }
};

struct CNode {
    enum class Tag {
        Population,
        Combinator,
    };

    struct Population_Body {
        NodeId id;
        const char *name;
        const char *related_constant_name;
        BoxedSlice<Link> links;
    };

    struct Combinator_Body {
        NodeId id;
        const char *name;
        uint32_t operation;
        BoxedSlice<NodeId> inputs;
    };

    Tag tag;
    union {
        Population_Body population;
        Combinator_Body combinator;
    };
};

struct CConstant {
    const char *name;
    double value;
};

struct CModel {
    MetaData meta_data;
    BoxedSlice<CNode> nodes;
    BoxedSlice<CConstant> constants;
};


extern "C" {

/// # Safety
/// `json_str` must be a valid pointer to a null-terminated C string. The
/// caller is responsible for freeing the returned `CModel` fields.
int model_from_cstring(const char *json_str, CModel *cmodel);

} // extern "C"
