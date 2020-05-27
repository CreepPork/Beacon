#include "script_component.hpp"
/*
 * Author: CreepPork
 * This would be called by the extension to log to the .rpt file.
 *
 * Arguments:
 * 0: Message to log <STRING>
 *
 * Return Value:
 * None
 *
 * Example:
 * ["My message"] call beacon_common_fnc_log
 *
 * Public: No
 */

params ["_message"];

INFO_1("%1",_message);
