cmake_minimum_required(VERSION 3.0.0)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(cmakeproj VERSION 0.1.0)


include(CTest)
enable_testing()

add_executable(main main.cpp)
# add_subdirectory(tests)
# add_subdirectory(utils)

#set(CPACK_PROJECT_NAME ${PROJECT_NAME})
#set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
#include(CPack)

find_package(imgui CONFIG REQUIRED)
target_link_libraries(main PRIVATE imgui::imgui)
