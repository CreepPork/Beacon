#include "script_component.hpp"
/*
 * Author: CreepPork
 * Bans a given Steam UID player from the server.
 *
 * Arguments:
 * 0: Server command password <STRING>
 * 1: Player Steam UID <STRING>
 *
 * Return Value:
 * Is Successful <BOOL>
 *
 * Example:
 * ["mypass", "|76561198054743530", "Very Bad!"] call beacon_commands_fnc_ban
 *
 * Public: No
 */

params [
    ["_password", "", [""]],
    ["_steamUid", "", [""]]
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

_password serverCommand format ["#exec ban %1", str _steamUid]
