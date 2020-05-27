#include "script_component.hpp"
/*
 * Author: CreepPork
 * Adds a message to the websocket buffer in the extension.
 *
 * Arguments:
 * 0: Message <STRING>
 *
 * Return Value:
 * None
 *
 * Example:
 * ["Hello socket!"] call beacon_server_fnc_message
 *
 * Public: No
 */

params [
    ["_message", "", [""]]
];

if (_message == "") exitWith {
    WARNING("Message cannot be empty.");
};

EXTENSION_NAME callExtension ["reply", _message];

nil
