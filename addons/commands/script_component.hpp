#define COMPONENT commands
#define COMPONENT_BEAUTIFIED Commands
#include "\x\beacon\addons\main\script_mod.hpp"

// #define DEBUG_MODE_FULL
// #define DISABLE_COMPILE_CACHE
// #define ENABLE_PERFORMANCE_COUNTERS

#ifdef DEBUG_ENABLED_COMMANDS
    #define DEBUG_MODE_FULL
#endif

#ifdef DEBUG_SETTINGS_COMMANDS
    #define DEBUG_SETTINGS DEBUG_SETTINGS_COMMANDS
#endif

#include "\x\beacon\addons\main\script_macros.hpp"

#define IS_CURATOR(unit) (unit call EFUNC(common,isCurator))
