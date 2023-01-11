cmake_minimum_required(VERSION 3.00)
project(breeze LANGUAGES C)

# Set project sources
FILE(GLOB LIBSOURCES ${CMAKE_SOURCE_DIR}/lib/*.c)

# Add project lib
add_library(PROJECTLIB SHARED ${LIBSOURCES})

# Add directory "lib/" to target library
target_include_directories(PROJECTLIB INTERFACE ${CMAKE_SOURCE_DIR}/lib)

# Create a binary executable
add_executable(breeze src/main.c)

# Link shared library to main
target_link_libraries(breeze PRIVATE PROJECTLIB)