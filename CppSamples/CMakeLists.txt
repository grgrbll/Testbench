cmake_minimum_required(VERSION 3.0.0)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(cmakeproj VERSION 0.1.0)

include(CTest)
enable_testing()

#add_executable(cmakeproj main.cpp)
add_subdirectory(tests)
#add_subdirectory(utils)
#add_subdirectory(imgui)

#set(CPACK_PROJECT_NAME ${PROJECT_NAME})
#set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
#include(CPack)

# this is heuristically generated, and may not be correct
#find_package(asio CONFIG REQUIRED)
#target_link_libraries(cmakeproj PRIVATE asio asio::asio)

# this is heuristically generated, and may not be correct
#find_package(GTest CONFIG REQUIRED)
#target_link_libraries(main PRIVATE GTest::gmock GTest::gtest GTest::gmock_main GTest::gtest_main)
