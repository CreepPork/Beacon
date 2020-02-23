params ["_name", "_code"];

{
	_result = [_name, name _x] call CBA_fnc_find;
    
	if (_result != -1) then {
		[_x, _code] remoteExec ["call", _x];
	};
} forEach allPlayers;