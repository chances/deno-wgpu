cmake_minimum_required(VERSION 3.9)

# Enable N-API C++ and JS Exception Handling
set(CMAKE_C_FLAGS "-fexceptions")
set(CMAKE_CXX_FLAGS "-std=c++11 -stdlib=libc++ -fexceptions")
set(CMAKE_XCODE_ATTRIBUTE_GCC_ENABLE_CPP_EXCEPTIONS "YES")
SET(CMAKE_XCODE_ATTRIBUTE_CLANG_CXX_LIBRARY "libc++")
set(CMAKE_OSX_DEPLOYMENT_TARGET "10.7")
add_compile_definitions(NAPI_CPP_EXCEPTIONS)

# Support macOS
IF(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
    set(cflags+ "-fvisibility=hidden")
    set(CMAKE_XCODE_ATTRIBUTE_GCC_SYMBOLS_PRIVATE_EXTERN "YES")
ENDIF(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")

#TODO: Support MSVS Settings: 'VCCLCompilerTool': { 'ExceptionHandling': 1 }

project (teraflop)

include_directories(${CMAKE_JS_INC})
file(GLOB SOURCE_FILES "src/*.cpp" "src/*.h")

# Include N-API wrappers
execute_process(COMMAND node -p "require('node-addon-api').include"
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_VARIABLE NODE_ADDON_API_DIR
        )
string(REPLACE "\"" "" NODE_ADDON_API_DIR ${NODE_ADDON_API_DIR})

add_definitions(-DNAPI_VERSION=3)

# Disable deprecated N-API APIs
add_compile_definitions(NODE_ADDON_API_DISABLE_DEPRECATED)

# Target Library

add_library(${PROJECT_NAME} SHARED ${SOURCE_FILES} ${CMAKE_JS_SRC})
set_target_properties(${PROJECT_NAME} PROPERTIES PREFIX "" SUFFIX ".node")

target_include_directories(${PROJECT_NAME} PRIVATE ${CMAKE_JS_INC})
target_include_directories(${PROJECT_NAME} PRIVATE ${NODE_ADDON_API_DIR})

target_link_libraries(${PROJECT_NAME} ${CMAKE_JS_LIB})
