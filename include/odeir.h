#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include <stdexcept>
#include <vector>
#include "opaque_rust_types.hpp"


struct MetaData {
    double start_time;
    double end_time;
    double delta_time;
};

using NodeId = uint32_t;

struct Link {
    uint32_t sign;
    uint32_t node_id;
};

template<typename T>
struct BoxedSlice {
    const T *ptr;
    size_t len;
    int (*destructor)(const BoxedSlice*);
    ~BoxedSlice() {
        if(destructor != nullptr && !destructor(this)) {
            std::printf("Failed to free BoxedSlice %p\n", this);
        }
    }

    BoxedSlice(): ptr(nullptr), len(0), destructor(nullptr) {}

    BoxedSlice(std::vector<T> vector): len(vector.size()) {
        T* mem = new T[len];
        std::copy(vector.begin(), vector.end(), mem);
        this->destructor = [](const BoxedSlice* slice) {
            delete[] slice->ptr;
            return 1;
        };
        this->ptr = mem;
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

struct Node {
    enum class Tag {
        Population,
        Combinator,
    };

    struct Population_Body {
        NodeId id;
        String name;
        String related_constant_name;
        BoxedSlice<Link> links;
    };

    struct Combinator_Body {
        NodeId id;
        String name;
        uint32_t operation;
        BoxedSlice<NodeId> inputs;
    };

    Tag tag;
    union {
        Population_Body population;
        Combinator_Body combinator;
    };

    ~Node() {
        switch (tag) {
            case Tag::Population: population.~Population_Body(); break;
            case Tag::Combinator: combinator.~Combinator_Body(); break;

        }
    }

    Node(const Node& other)
     : tag(other.tag) {
        switch (tag) {
            case Tag::Population: ::new (&population) (Population_Body)(other.population); break;
            case Tag::Combinator: ::new (&combinator) (Combinator_Body)(other.combinator); break;

        }
    }
};

template<typename K, typename V>
struct HashWrapper {
    Box<HashMap<K, V>> _0;
};

struct Constant {
    String name;
    double value;
};

struct Model {
    MetaData meta_data;
    HashWrapper<NodeId, Node> nodes;
    BoxedSlice<Constant> constants;
};


extern "C" {

Model model_from_cstr(const char *json_cstr);

} // extern "C"
