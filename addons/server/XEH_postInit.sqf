#include "script_component.hpp"

GVAR(started) = false;

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_function", "_data"];

    if (_name == "beacon") then {
        [_function, _data] call beacon_server_fnc_callbackRouter;
    };

    diag_log(format ["found callback: %1, %2, %3", _name, _function, _data]);
}];

diag_log "starting";

[] call beacon_server_fnc_start;

diag_log "started";
diag_log GVAR(started);
