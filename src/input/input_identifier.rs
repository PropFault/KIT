pub enum JoyButtonsCommon{
    A = 0,
    B = 1,
    X = 2,
    Y = 3,
    DpUp = 4,
    DpDown = 5,
    DpLeft = 6,
    DpRight = 7,
    LS = 8,
    RS = 9,
    LT = 10,
    RT = 11,
    Start = 12,
    Select = 13,
    GripL = 14,
    GripR = 15,
    Share = 16,
    Options = 17,
    StickL = 18,
    StickR = 19,
    Special1 = 20,
    Special2 = 21,
    Special3 = 22
}

pub enum JoyAxisCommon{
    L = 0,
    R = 1,
    LT = 2,
    RT = 3
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum InputIdentifier{
    Joy{ axis_number : i8, joy_number : i8},
    Key{ key : char },
    Button{ button_num : i8},
    Action{ id : String }
}