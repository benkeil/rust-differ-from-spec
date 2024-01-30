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

impl<T: DifferFromSpec + PartialEq> DifferFromSpec for Option<T> {
    fn differ_from_spec(&self, spec: &Self) -> bool {
        spec.is_some() && spec != self
    }
}
