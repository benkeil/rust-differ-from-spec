use std::collections::HashSet;
use std::hash::Hash;

pub trait DifferFromSpec {
    fn differ_from_spec(&self, spec: &Self) -> bool;
}

impl DifferFromSpec for String {
    fn differ_from_spec(&self, spec: &Self) -> bool {
        spec != self
    }
}

impl DifferFromSpec for bool {
    fn differ_from_spec(&self, spec: &Self) -> bool {
        spec != self
    }
}

impl DifferFromSpec for u8 {
    fn differ_from_spec(&self, spec: &Self) -> bool {
        spec != self
    }
}

impl DifferFromSpec for u32 {
    fn differ_from_spec(&self, spec: &Self) -> bool {
        spec != self
    }
}

impl DifferFromSpec for u64 {
    fn differ_from_spec(&self, spec: &Self) -> bool {
        spec != self
    }
}

impl<T: DifferFromSpec + Eq + Hash> DifferFromSpec for Vec<T> {
    fn differ_from_spec(&self, spec: &Self) -> bool {
        self.len() != spec.len() || {
            let set = spec.iter().collect::<HashSet<_>>();
            !self.iter().all(|item| set.contains(item))
        }
    }
}

impl<T: DifferFromSpec> DifferFromSpec for Option<T> {
    fn differ_from_spec(&self, spec: &Self) -> bool {
        match (spec, self) {
            (Some(some_spec), Some(some_actual)) => some_actual.differ_from_spec(some_spec),
            (Some(_), None) => true,
            _ => false,
        }
    }
}
