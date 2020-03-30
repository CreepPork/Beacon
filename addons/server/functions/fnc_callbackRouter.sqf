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

params ["_functionName", "_data"];

_parsedData = parseSimpleArray _data;
_function = compile _functionName;

if (isNil "_function") exitWith {
    WARNING_1("The given function %1 does not exist.",_functionName);
};

_parsedData call _function;

nil
