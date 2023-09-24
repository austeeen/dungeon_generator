cmake_minimum_required(VERSION 3.16.3)

project(${APP_NAME})

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "-Wall -ggdb3")
set(CMAKE_BUILD_TYPE Debug)

if (WIN32)
    # for Windows operating system in general
    set(SFML_ROOT_PATH "C:/msys64/home/pants/SFML-2.5.1")
    set(SFML_DIR ${SFML_ROOT_PATH}/lib/cmake/SFML)
    set(SFML_DLL_DIR ${SFML_ROOT_PATH}/bin)
endif()

file(GLOB SQR_PROJECT_DEPS ${CMAKE_BINARY_DIR}/../deps/*.cpp)

add_subdirectory(lib)
add_subdirectory(app/${APP_NAME})