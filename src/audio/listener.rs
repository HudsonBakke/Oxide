use std::sync::{Arc, Mutex, OnceLock};
use crate::math::Vec3;
use super::{AudioResult, direction_valid, invalid, lock, position_valid};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ListenerData { pub position: Vec3, pub forward: Vec3, pub up: Vec3 }
impl Default for ListenerData {
    fn default() -> Self { Self { position: Vec3::default(), forward: Vec3::new(0.0,0.0,-1.0), up: Vec3::new(0.0,1.0,0.0) } }
}
static CURRENT: OnceLock<Mutex<Arc<Mutex<ListenerData>>>> = OnceLock::new();
fn current() -> &'static Mutex<Arc<Mutex<ListenerData>>> {
    CURRENT.get_or_init(|| Mutex::new(Arc::new(Mutex::new(ListenerData::default()))))
}
pub(crate) fn snapshot() -> ListenerData { *lock(&lock(current())) }

/// Position and orientation from which all positional audio is heard.
///
/// Call activate once to select this listener. Subsequent edits apply live.
/// The active listener is retained until another is activated; it is independent
/// of the rendering camera. The initial listener is at the origin facing -Z.
#[derive(Debug, Clone)]
pub struct AudioListener { data: Arc<Mutex<ListenerData>> }
impl Default for AudioListener {
    fn default() -> Self { Self { data: Arc::new(Mutex::new(ListenerData::default())) } }
}
impl AudioListener {
    /// Creates a listener at the origin facing -Z with +Y up; does not activate it.
    pub fn new() -> Self { Self::default() }
    /// Selects this listener; later changes are audible without reactivation.
    pub fn activate(&self) { *lock(current()) = self.data.clone(); }
    /// World-space listener position.
    pub fn position(&self) -> Vec3 { lock(&self.data).position }
    /// Changes the world-space position; rejects nonfinite or extreme components.
    pub fn set_position(&self, position: Vec3) -> AudioResult<()> {
        position_valid(position)?; lock(&self.data).position = position; Ok(())
    }
    /// Unit vector pointing forward, initially -Z.
    pub fn direction(&self) -> Vec3 { lock(&self.data).forward }
    /// Unit up vector, orthogonal to the forward vector.
    pub fn up(&self) -> Vec3 { lock(&self.data).up }
    /// Sets orientation, normalizing and orthogonalizing nonzero, nonparallel vectors.
    pub fn set_orientation(&self, forward: Vec3, up: Vec3) -> AudioResult<()> {
        let forward = direction_valid(forward)?;
        let up = direction_valid(up)?;
        let right = forward.cross(&up);
        if right.length() < 1.0e-5 { return Err(invalid("listener forward and up must not be parallel")); }
        let mut data = lock(&self.data);
        data.forward = forward; data.up = right.normalized().cross(&forward).normalized();
        Ok(())
    }
}
