#include "script_component.hpp"

GVAR(started) = false;

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_function", "_data"];

    if (_name == "beacon") then {
        [_function, _data] call FUNC(callbackRouter);
    };

    LOG_3("Found callback: %1, %2, %3",_name,_function,_data);
}];

[] call FUNC(start);

