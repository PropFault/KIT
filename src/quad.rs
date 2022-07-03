pub trait Quad{
    fn set_width(&self, width: u16) -> bool;
    fn set_height(&self, height: u16) -> bool;
    fn set_dimens(&self, width: u16, height: u16) -> bool;
    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;
    fn set_x(&self, x: u16) -> bool;
    fn set_y(&self, y: u16) -> bool;
    fn set_pos(&self, x: u16, y: u16) -> bool;
    fn get_x(&self) -> u16;
    fn get_y(&self) -> u16;
}