//! mtvec register

const MASK: usize = usize::MAX;
const TRAP_MASK: usize = 0b11;

read_write_csr! {
    /// mtvec register
    Mtvec: 0x305,
    mask: MASK,
}

csr_field_enum! {
    /// Trap mode
    TrapMode {
        default: Direct,
        Direct = 0,
        Vectored = 1,
    }
}

read_write_csr_field! {
    Mtvec,
    /// Accesses the trap-vector mode..
    trap_mode,
    TrapMode: [0:1],
}

impl Mtvec {
    /// Returns the trap-vector base-address
    #[inline]
    pub const fn address(&self) -> usize {
        self.bits - (self.bits & TRAP_MASK)
    }

    /// Sets the trap-vector base-address.
    ///
    /// # Note
    ///
    /// The address is aligned to 4-bytes.
    #[inline]
    pub fn set_address(&mut self, address: usize) {
        self.bits = (address & !TRAP_MASK) | (self.bits & TRAP_MASK);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mtvec() {
        let mut m = Mtvec::from_bits(0);

        (1..=usize::BITS)
            .map(|r| (((1u128 << r) - 1) as usize))
            .for_each(|address| {
                m.set_address(address);
                assert_eq!(m.address(), address & !TRAP_MASK);
            });

        test_csr_field!(m, trap_mode: TrapMode::Direct);
        test_csr_field!(m, trap_mode: TrapMode::Vectored);
    }
}
