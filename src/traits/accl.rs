use num_traits::Num;
use quaternion_core::Vector3;

pub trait Accl<N>
where
    N: Num,
{
    fn read_accl(&mut self);
    fn get_accl(&self) -> Vector3<N>;
}
