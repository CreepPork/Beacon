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

param ["_message"];

diag_log format ["Beacon: %1", _message];
