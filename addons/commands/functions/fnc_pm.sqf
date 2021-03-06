#include "script_component.hpp"
/*
 * Author: CreepPork
 * Sends a personal message to a single player.
 *
 * Arguments:
 * 0: Player Steam UID <STRING>
 * 1: Message <STRING>
 * 2: Footer <STRING>
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
    ["_message", "", [""]],
    ["_footer", "", [""]]
];

if (_message == "") exitWith {
    WARNING_1("Message %1 cannot be empty.");
};

if (_steamUid == "") exitWith {
    WARNING("Steam UID was not passed.");
};

// Drop the |
_steamUid = _steamUid select [1];

private _unit = _steamUid call EFUNC(common,getUnitByUid);

if (isNull _unit) exitWith {
    WARNING_1("Given Steam UID (%1) is not in-game!",_steamUid);
    false
};

[
    "[SERVER] [DM]",
    _message,
    _footer
] remoteExecCall [QEFUNC(common,showMessageHint), _unit];

nil
