#include "script_component.hpp"
/*
 * Author: CreepPork
 * Gets all players and returns their name, Steam UID and if they are Zeus.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * All players present <ARRAY>
 *
 * Example:
 * [] call beacon_commands_fnc_getPlayers
 *
 * Public: No
 */

[] call CBA_fnc_players apply { [name _x, getPlayerUID _x, IS_CURATOR(_x)] }
