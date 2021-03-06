#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        name = COMPONENT_NAME;
        units[] = {};
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {"cba_main"};
        author = CSTRING(Author);
        authors[] = {"CreepPork"};
        url = CSTRING(URL);
        VERSION_CONFIG;
    };
};

#include "CfgSettings.hpp"
