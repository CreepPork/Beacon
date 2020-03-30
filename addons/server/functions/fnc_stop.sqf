#include "script_component.hpp"
/*
 * Author: CreepPork
 * Stops the extension socket server.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * None
 *
 * Example:
 * [] call beacon_server_fnc_stop
 *
 * Public: No
 */

EXTENSION_NAME callExtension "stop";

GVAR(started) = false;
