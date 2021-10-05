private _unit = group player createUnit ["B_RangeMaster_F", position player, [], 0, "FORM"];
if (side _unit isEqualTo blufor) then {
    _unit addWeapon "arifle_MXC_F";
} else {
    _unit addWeapon "arifle_Katiba_GL_F";
};
_unit
