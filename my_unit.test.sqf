private _unit = call testmod_common_fnc_my_unit;

[side _unit, side player] call hemtt_test_fnc_assertEq;
[group _unit, group player] call hemtt_test_fnc_assertEq;
[primaryWeapon _unit, "arifle_MXC_F"] call hemtt_test_fnc_assertEq;
[alive _unit] call hemtt_test_fnc_assert;
