#include "script_component.hpp"

if (isServer) then {
    GVAR(customChannel) = radioChannelCreate [
        [1, 0, 0, 1],
        "[SERVER]",
        "[SERVER]",
        [] call CBA_fnc_players,
        false
    ];

    [
        GVAR(customChannel),
        { _this radioChannelAdd [player] }
    ] remoteExec ["call", [0, -2] select isDedicated, "[SERVER]"];
};
