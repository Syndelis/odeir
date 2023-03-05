#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


struct MetaData {
    double start_time;
    double end_time;
    double delta_time;
};

using NodeId = uint32_t;

template<typename T>
struct RawVec {
    const T *ptr;
    size_t len;
    int (*destructor)(RawVec);
    ~RawVec<T>() {
        if(!destructor(*this)) {
            std::printf("Failed to free RawVec %p\n", this);
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
    };

    struct Combinator_Body {
        NodeId id;
        const char *name;
        uint32_t operation;
        RawVec<NodeId> inputs;
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
    RawVec<CNode> nodes;
    RawVec<CConstant> constants;
};


extern "C" {

int model_from_cstring(const char *json_str, CModel *cmodel);

} // extern "C"
