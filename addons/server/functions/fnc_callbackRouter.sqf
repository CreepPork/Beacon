#include "script_component.hpp"
/*
 * Author: CreepPork
 * Routes callbacks from the extension to their functions.
 *
 * Arguments:
 * 0: Function to route to <STRING>
 * 1: Provided data <STRING>
 *
 * Return Value:
 * None
 *
 * Example:
 * ["some_function_to_call", "[true, 0.5]"] call beacon_server_fnc_callbackRouter
 *
 * Public: No
 */

params [
    ["_functionName", "", [""]],
    ["_data", "", [""]]
];

if (_functionName == "") exitWith {
    WARNING_1("Function name %1 cannot be empty.",_functionName);
};

private _parsedData = "";

if (_data select [0, 1] == "[") then {
    _parsedData = parseSimpleArray _data;
} else {
    _parsedData = _data;
};

private _function = compile _functionName;

if (isNil { call _function }) exitWith {
    WARNING_1("The given function %1 does not exist.",_functionName);
};

// We have a double-call here because otherwise it returns the code itself
private _result = _parsedData call call _function;

if (isNil "_result") exitWith {};

[
    format ["Result (%1): %2", _functionName, _result]
] call FUNC(message);
