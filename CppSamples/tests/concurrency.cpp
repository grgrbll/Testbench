#include <gtest/gtest.h>
#include <thread>
#include "lib/active_object.cpp"

void task(std::string s)
{

}

TEST(concurrency, thread)
{   
    auto t = std::thread(task, "hello");
}

TEST(concurrency, active_object)
{
    ActiveObject A;
    A.RequestAdd(1,2);
}