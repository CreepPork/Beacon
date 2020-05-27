#include "script_component.hpp"
/*
 * Author: CreepPork
 * Says a message to all clients from the server.
 *
 * Arguments:
 * 0: Message <STRING>
 *
 * Return Value:
 * None
 *
 * Example:
 * ["Server is closing."] call beacon_commands_fnc_say
 *
 * Public: No
 */

params [
    ["_message", "", [""]]
];

if (_message == "") exitWith {
    WARNING_1("Message %1 cannot be empty.");
};

if (GVAR(customChannel) == 0) exitWith {
    WARNING("Custom channel could not be created.");
};

{
} forEach allPlayers;

LOG_1("Server said: %1",_message);
