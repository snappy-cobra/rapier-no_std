extern crate alloc;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

pub use self::impulse_joint::ImpulseJoint;
pub use self::impulse_joint_set::{ImpulseJointHandle, ImpulseJointSet};
pub(crate) use self::impulse_joint_set::{JointGraphEdge, JointIndex};

mod impulse_joint;
mod impulse_joint_set;
