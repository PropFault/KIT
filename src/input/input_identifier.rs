pub enum InputType{
    KEY,
    JOY,
    ACTION
}
pub struct InputBodyJoy{
    pub axis_number : i8,
    pub joy_number : i8
}
pub struct InputBodyKey{
    pub key : char
}
pub struct InputBodyAction{
    pub id : String
}

enum InputBody{
    Joy(InputBodyJoy),
    Key(InputBodyKey),
    Action(InputBodyAction)
}
pub struct InputIdentifier{
    pub input_type : InputType,
    pub body : InputBody
}