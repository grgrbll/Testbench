#include <gtest/gtest.h>
#include <immer/vector.hpp>
#include <immer/flex_vector.hpp>
#include <immer/box.hpp>

TEST(immer, vector)
{
    auto v0 = immer::vector<int>{};
    auto v1 = v0.push_back(13);

    ASSERT_TRUE(v0.size() == 0 && v1.size() == 1 && v1[0] == 13);

    auto v2 = v1.set(0, 42);

    ASSERT_TRUE(v1[0] == 13 && v2[0] == 42);
}

TEST(immer, flex_vector_take_and_drop)
{
    auto v0 = immer::flex_vector<int>{0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    auto v1 = v0.drop(2).take(3);
    ASSERT_TRUE(v1.size() == 3 &&
                v1[0] == 2 &&
                v1[1] == 3 &&
                v1[2] == 4);

    auto v2 = v1.set(0, 42);

    ASSERT_TRUE(v1[0] == 13 && v2[0] == 42);
}

TEST(immer, box_update)
{
    auto v1 = immer::box<std::string>{"hello"};
    auto v2 = v1.update([&](auto l)
                        { return l + ", world!"; });

    ASSERT_TRUE(v1 == immer::box<std::string>{"hello"});
    ASSERT_TRUE(v2 == immer::box<std::string>{"hello, world!"});
}

TEST(immer, box_assign)
{
    auto v1 = immer::box<std::string>{"hello"};
    auto v2 = v1;
    v2 = "hello, world!";

    ASSERT_TRUE(v1 == immer::box<std::string>{"hello"});
    ASSERT_TRUE(v2 == immer::box<std::string>{"hello, world!"});
}