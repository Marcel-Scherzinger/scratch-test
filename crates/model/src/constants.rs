use crate::interpret_json::OpcodeNum;
// I'm not sure what a "shadow" is but it looks like it
// can be ignored for this use case
// I assume that it is the value the block would have by default,
// the value it would get after moving a block out of this position
// If this interpretation is correct shadows CAN be ignored.
pub const INPUT_SAME_BLOCK_SHADOW: OpcodeNum = 1; // unobscured shadow
pub const INPUT_BLOCK_NO_SHADOW: OpcodeNum = 2; // no shadow
pub const INPUT_DIFF_BLOCK_SHADOW: OpcodeNum = 3; // obscured shadow
// Constants referring to 'primitive' blocks that are usually shadows,
// or in the case of variables and lists, appear quite often in projects
// math_number
pub const MATH_NUM_PRIMITIVE: OpcodeNum = 4; // there's no reason these constants can't collide
// math_positive_number
pub const POSITIVE_NUM_PRIMITIVE: OpcodeNum = 5; // with the above, but removing duplication for clarity
// math_whole_number
pub const WHOLE_NUM_PRIMITIVE: OpcodeNum = 6;
// math_integer
pub const INTEGER_NUM_PRIMITIVE: OpcodeNum = 7;
// math_angle
pub const ANGLE_NUM_PRIMITIVE: OpcodeNum = 8;
// colour_picker
pub const COLOR_PICKER_PRIMITIVE: OpcodeNum = 9;
// text
pub const TEXT_PRIMITIVE: OpcodeNum = 10;
// event_broadcast_menu
pub const BROADCAST_PRIMITIVE: OpcodeNum = 11;
// data_variable
pub const VAR_PRIMITIVE: OpcodeNum = 12;
// data_listcontents
pub const LIST_PRIMITIVE: OpcodeNum = 13;
