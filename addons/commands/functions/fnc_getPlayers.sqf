#include "script_component.hpp"
/*
 * Author: CreepPork
 * Gets all players and returns their name and Steam UID.
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

allPlayers apply { [name _x, getPlayerUID _x] }
