//! `mcountinhibit` register

use crate::read_write_csr;

read_write_csr!(
    "`mcountinhibit` register",
    Mcountinhibit,
    0x320,
    [
        {
            "Gets the `cycle[h]` inhibit field value.", cy,
            "Sets the `cycle[h]` inhibit field value.\n\n**NOTE**: only updates the in-memory value without touching the CSR.", set_cy,
            0
        },
        {
            "Gets the `instret[h]` inhibit field value.", ir,
            "Sets the `instret[h]` inhibit field value.\n\n**NOTE**: only updates the in-memory value without touching the CSR.", set_ir,
            2
        },
        {
            "Gets the `mhpmcounterX[h]` inhibit field value.\n\n**WARN**: `index` must be in the range `[31:3]`.", hpm,
            "Sets the `mhpmcounterX[h]` inhibit field value.\n\n**WARN**: `index` must be in the range `[31:3]`.\n\n**NOTE**: only updates the in-memory value without touching the CSR.", set_hpm,
            3, 31
        }
    ]
); 

set_clear_csr!(
/// Machine cycle Disable
    , set_cy, clear_cy, 1 << 0);

set_clear_csr!(
/// Machine instret Disable
    , set_ir, clear_ir, 1 << 2);

#[inline]
pub unsafe fn set_hpm(index: usize) {
    assert!((3..32).contains(&index));
    _set(1 << index);
}

#[inline]
pub unsafe fn clear_hpm(index: usize) {
    assert!((3..32).contains(&index));
    _clear(1 << index);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcountinhibit() {
        let mut m = Mcountinhibit { bits: 0 };

        assert!(!m.cy());

        m.set_cy(true);
        assert!(m.cy());

        m.set_cy(false);
        assert!(!m.cy());

        assert!(!m.ir());

        m.set_ir(true);
        assert!(m.ir());

        m.set_ir(false);
        assert!(!m.ir());

        (3..32).for_each(|i| {
            assert!(!m.hpm(i));

            m.set_hpm(i, true);
            assert!(m.hpm(i));

            m.set_hpm(i, false);
            assert!(!m.hpm(i));
        });
    }
}
