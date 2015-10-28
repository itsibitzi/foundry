pub trait Buffer {
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_to(&mut self, line: u64, col: u64)

    fn get_line(&self, line: u64) -> &str;
}
