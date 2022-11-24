cmake_minimum_required(VERSION 3.23)
project(vm_manager)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED true)
set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -D_GLIBCXX_DEBUG")
set(CMAKE_CXX_STANDARD 17)

add_compile_options(
		-g -Wall -Werror -Wextra -pthread
)

add_link_options(-lpthread -lm -v -pthread)

add_executable(vm_manager main.cpp)
