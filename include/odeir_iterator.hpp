#ifndef ODEIR_ITERATOR_HPP
#define ODEIR_ITERATOR_HPP

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

#endif
