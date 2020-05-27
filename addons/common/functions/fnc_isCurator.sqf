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

params ["_unit"];

_unit in (allCurators apply { getAssignedCuratorUnit _x });
