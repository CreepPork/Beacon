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

LOG_2("Data is %1, type %2",_data,typeName _data);

// If data isn't string array then we want to wrap it in an array
private _parsedData = if (_data select [0] == "[") then {
    parseSimpleArray _data
} else {
    parseSimpleArray format ["[%1]", str _data]
};

LOG_2("Data is %1, type %2",_parsedData,typeName _parsedData);

{
    LOG_3("Data (%1): %2; type: %3",_forEachIndex,_x,typeName _x);    
} forEach _parsedData;

private _function = compile _functionName;

if (isNil "_function") exitWith {
    WARNING_1("The given function %1 does not exist.",_functionName);
};

LOG_3("Function: %1, %2, %3",_functionName,_function,_parsedData call _function);

_parsedData call _function;

nil
