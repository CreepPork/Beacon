#define COMPONENT server
#define COMPONENT_BEAUTIFIED SERVER
#include "\x\beacon\addons\main\script_mod.hpp"

// #define DEBUG_MODE_FULL
// #define DISABLE_COMPILE_CACHE
// #define ENABLE_PERFORMANCE_COUNTERS

#ifdef DEBUG_ENABLED_SERVER
    #define DEBUG_MODE_FULL
#endif

#ifdef DEBUG_SETTINGS_SERVER
    #define DEBUG_SETTINGS DEBUG_SETTINGS_SERVER
#endif

#include "\x\beacon\addons\main\script_macros.hpp"
