#include "script_component.hpp"
/*
 * Author: CreepPork
 * Sends a personal message to a single player.
 *
 * Arguments:
 * 0: Player Steam UID <STRING>
 * 1: Message <STRING>
 *
 * Return Value:
 * None
 *
 * Example:
 * ["|76561198054743530", "Stop that!"] call beacon_commands_fnc_pm
 *
 * Public: No
 */

params [
    ["_steamUid", "", [""]],
    ["_message", "", [""]]
];

if (_message == "") exitWith {
    WARNING_1("Message %1 cannot be empty.");
};

if (_steamUid == "") exitWith {
    WARNING("Steam UID was not passed.");
};

if (GVAR(customChannel) == 0) exitWith {
    WARNING("Custom channel could not be created.");
};

// Drop the |
_steamUid = _steamUid select [1];

private _unit = _steamUid call EFUNC(common,getUnitByUid);

if (isNull _unit) exitWith {
    WARNING_1("Given Steam UID (%1) is not in-game!",_steamUid);
    false
};

[_unit, [GVAR(customChannel), _message]] remoteExecCall ["customChat", _unit];

nil
