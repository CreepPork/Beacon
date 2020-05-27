#include "script_component.hpp"
/*
 * Author: CreepPork
 * Kicks a given Steam UID player from the server.
 *
 * Arguments:
 * 0: Server command password <STRING>
 * 1: Player Steam UID <NUMBER>
 * 2: Message to show to the player <STRING>
 *
 * Return Value:
 * Is Successful <BOOL>
 *
 * Example:
 * ["mypass", 76561198054743530, "Bad!"] call beacon_commands_fnc_kick
 *
 * Public: No
 */

params [
    ["_password", "", [""]],
    ["_steamUid", "", [""]],
    ["_message", "", [""]]
];

if (_password == "") exitWith {
    WARNING("Server command password was empty.");
    false
};

if (_steamUid == "") exitWith {
    WARNING("Steam UID was not passed.");
    false
};

// Drop the |
_steamUid = _steamUid select [1];

if (isNull (_steamUid call EFUNC(common,getUnitByUid))) exitWith {
    WARNING_1("Given Steam UID (%1) is not in-game!",_steamUid);
    false
};

_password serverCommand format ["#kick %1 %2", _steamUid, _message]
