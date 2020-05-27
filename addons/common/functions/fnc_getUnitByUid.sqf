#include "script_component.hpp"
/*
 * Author: CreepPork
 * Finds a player unit by the given Steam UID.
 *
 * Arguments:
 * 0: Steam UID <STRING>
 *
 * Return Value:
 * Player unit <OBJECT>
 *
 * Example:
 * ["76561198054743530"] call beacon_common_fnc_getUnitByUid
 *
 * Public: No
 */

params [
    ["_uid", "", [""]]
];

if (_uid == "") exitWith {
    WARNING("Steam UID cannot be empty.");
};

private _unit = objNull;

{
    if (getPlayerUID _x == _uid) exitWith {
        _unit = _x;
    };
} forEach allPlayers;

_unit
