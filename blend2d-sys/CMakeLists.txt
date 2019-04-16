cmake_minimum_required(VERSION 3.1)

set(CMAKE_CXX_STANDARD 17)

set(BLEND2D_DIR "${CMAKE_CURRENT_LIST_DIR}/blend2d"
        CACHE PATH "Location of 'blend2d'")

set(BLEND2D_BUILD_STATIC TRUE)
include("${BLEND2D_DIR}/CMakeLists.txt")

list(REMOVE_ITEM BLEND2D_LIBS "blend2d")

target_include_directories(blend2d BEFORE PUBLIC ${BLEND2D_INCLUDE_DIR})
target_compile_options(blend2d PUBLIC ${BLEND2D_CFLAGS})
target_link_libraries(blend2d ${BLEND2D_LIBS})
install(TARGETS blend2d ARCHIVE DESTINATION lib)