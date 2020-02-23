#include "script_component.hpp"

addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_function", "_data"];

    diag_log(format ["found callback: %1, %2, %3", _name, _function, _data]);
}];

"beacon" callExtension "start_server";