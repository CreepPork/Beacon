#include "script_component.hpp"
/*
 * Author: CreepPork
 * Displays a hint which is formatted as a big, intrusive message.
 *  https://gyazo.com/567ab60ca870d80a628e85491d45401f
 *
 * Arguments:
 * 0: Title <STRING>
 * 1: Body <STRING>
 * 2: Footer message <STRING>
 *
 * Return Value:
 * None
 *
 * Example:
 * ["Server", "My test message."] call beacon_common_fnc_showMessageHint
 *
 * Public: No
 */

params ["_title", "_body", ["_footer", "", [""]]];

_title = toUpper _title;

hint parseText format [
    "<t size='2'><t size='2' valign='top' align='center' color='#ff0000'>%1</t> <br/> <t size='1' valign='middle' align='center'>%2</t> <br/> <t size='0.8' color='#C0C0C0' valign='bottom' align='center'>%3</t></t>",
    _title,
    _body,
    _footer
];
