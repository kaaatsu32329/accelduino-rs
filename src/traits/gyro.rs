use num_traits::Num;
use quaternion_core::Vector3;

pub trait Gyro<N>
where
    N: Num,
{
    fn read_gyro(&mut self);
    fn get_gyro(&self) -> Vector3<N>;
}
