pub trait Interface {
    type Error;

    fn cmd_0(&mut self, cmd: u8) -> Result<(), Self::Error>;
    fn cmd_1(&mut self, cmd: u8, a: u8) -> Result<(), Self::Error>;
    fn cmd_2(&mut self, cmd: u8, a: u8, b: u8) -> Result<(), Self::Error>;
    fn cmd_n(&mut self, cmd: u8, data: &[u8]) -> Result<(), Self::Error>;
    fn cmd_n_iter<I: core::iter::IntoIterator<Item = u8>>(
        &mut self,
        cmd: u8,
        data: I,
    ) -> Result<usize, Self::Error>;
}
