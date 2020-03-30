#include "script_component.hpp"
/*
 * Author: CreepPork
 * Starts the extension socket server.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * None
 *
 * Example:
 * [] call beacon_server_fnc_start
 *
 * Public: No
 */

EXTENSION_NAME callExtension "start";

GVAR(started) = true;
