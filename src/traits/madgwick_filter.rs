use num_traits::Float;
use quaternion_core::Vector3;

pub trait MadgwickFilter<F>
where
    F: Float,
{
    fn set_sampling_time(&mut self, time: u16);
    fn raw_vector(&self) -> Vector3<F>;
    fn update(&self) -> Vector3<F>;

    fn calculate(&self, target: Vector3<F>, sampling_time: u16) -> Vector3<F> {
        todo!()
    }
}
