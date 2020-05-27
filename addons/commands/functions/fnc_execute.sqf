#include "script_component.hpp"
/*
 * Author: CreepPork
 * Executes code that is passed to it.
 *
 * Arguments:
 * 0: Code to execute <STRING>
 *
 * Return Value:
 * Executed code returns <ANY>
 *
 * Example:
 * ["hint ""hello world""] call beacon_commands_fnc_execute
 *
 * Public: No
 */

params [
    ["_code", "", [""]]
];

if (_code == "") exitWith {
    WARNING_1("Code %1 must be a string.",_code);
};

call compile _code
