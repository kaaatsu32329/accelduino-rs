use num_traits::Num;
use quaternion_core::Vector3;

pub trait Mag<N>
where
    N: Num,
{
    fn read_mag(&mut self);
    fn get_mag(&self) -> Vector3<N>;
}
