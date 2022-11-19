cmake_minimum_required(VERSION 3.23)
project(multi_thread_addon C)

set(CMAKE_C_STANDARD 11)

add_executable(make_chaos make_chaos_main.c)
add_executable(mutex_sol mutex_sol.c)
add_executable(peterson_sol peterson_sol.c)
