#include "script_component.hpp"
/*
 * Author: CreepPork
 * Returns if the given unit is a Curator.
 *
 * Arguments:
 * 0: Unit to test <OBJECT>
 *
 * Return Value:
 * Is a Curator <BOOL>
 *
 * Example:
 * [player] call beacon_common_fnc_isCurator
 *
 * Public: No
 */

params [
    ["_unit", objNull, [objNull]]
];

if (isNull _unit) exitWith {
    WARNING("Unit cannot be null.");
};

_unit in (allCurators apply { getAssignedCuratorUnit _x })
