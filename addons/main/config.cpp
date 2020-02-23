#include "script_component.hpp"

class CfgPatches {
	class Beacon {
		name = "Beacon";
		units[] = {};
		weapons[] = {};
		requiredVersion = 1.88;
		requiredAddons[] = {"intercept_core"};
		author = "CreepPork";
		authors[] = {"CreepPork"};
		url = "https://github.com/CreepPork/Beacon";
		version = "1.0";
		versionStr = "1.0";
		versionAr[] = {1,0};
	};
};

class Intercept {
    class CreepPork {
        class Beacon {
            pluginName = "beacon";
        };
    };
};

class Extended_PreStart_EventHandlers {
    class ADDON {
        init = QUOTE(call COMPILE_FILE(XEH_preStart));
    };
};

class Extended_PreInit_EventHandlers {
    class ADDON {
        init = QUOTE(call COMPILE_FILE(XEH_preInit));
    };
};