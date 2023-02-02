cmake_minimum_required(VERSION 3.00)
project(breeze LANGUAGES C)

# Set project sources
FILE(GLOB LIBSOURCES ${CMAKE_SOURCE_DIR}/lib/*/*.c)

# find flex and bison
find_package(BISON REQUIRED)
set(BisonOutput ${CMAKE_SOURCE_DIR}/lib/parser.c)
set(BisonInput ${CMAKE_SOURCE_DIR}/lib/parser.y)
if(BISON_FOUND)
    add_custom_command(
        OUTPUT ${BisonOutput}
        COMMAND ${BISON_EXECUTABLE}
                --defines=${CMAKE_SOURCE_DIR}/lib/y.tab.h
                --output=${BisonOutput}
                ${BisonInput}
        COMMENT "Generating parser.c"
    )
endif()

find_package(FLEX REQUIRED)
set(FlexOutput ${CMAKE_SOURCE_DIR}/lib/scanner.c)
set(FlexInput ${CMAKE_SOURCE_DIR}/lib/scanner.l)
if(FLEX_FOUND)
    add_custom_command(
        OUTPUT ${FlexOutput}
        COMMAND ${FLEX_EXECUTABLE}
                --outfile=${FlexOutput}
                ${FlexInput}
        COMMENT "Generating scanner.c"
    )
endif()

# Add project lib
add_library(Breeze SHARED ${LIBSOURCES} ${FlexOutput} ${BisonOutput} ${FlexInput} ${BisonInput})

# Add directory "lib/" to target library
target_include_directories(Breeze INTERFACE ${CMAKE_SOURCE_DIR}/lib)

# Create a binary executable
add_executable(breeze src/main.c)

# Link shared library to main
target_link_libraries(breeze PRIVATE Breeze)