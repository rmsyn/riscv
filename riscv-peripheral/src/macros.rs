//! Utility macros for generating standard peripherals-related code in RISC-V PACs.

/// Macro to create interfaces to CLINT peripherals in PACs.
/// The resulting struct will be named `CLINT`, and will provide safe access to the CLINT registers.
///
/// This macro expects 5 different argument types:
///
/// - Base address (**MANDATORY**): base address of the CLINT peripheral of the target.
/// - Frequency (**OPTIONAL**): clock frequency (in Hz) of the `MTIME` register. It enables the `delay` method of the `CLINT` struct.
/// - Async flag (**OPTIONAL**): It enables the `async_delay` method of the `CLINT struct`.
///   You must activate the `embedded-hal-async` feature to use this flag.
/// - Per-HART mtimecmp registers (**OPTIONAL**): a list of `mtimecmp` registers for easing access to per-HART mtimecmp regs.
/// - Per-HART msip registers (**OPTIONAL**): a list of `msip` registers for easing access to per-HART msip regs.
///
/// Check the examples below for more details about the usage and syntax of this macro.
///
/// # Example
///
/// ## Base address only
///
/// ```
/// riscv_peripheral::clint_codegen!(base 0x0200_0000, freq 32_768,); // do not forget the ending comma!
///
/// let mswi = CLINT::mswi();     // MSWI peripheral
/// let mtimer = CLINT::mtimer(); // MTIMER peripheral
/// let delay = CLINT::delay();   // For the `embedded_hal::delay::DelayNs` trait
/// ```
///
/// ## Base address and per-HART mtimecmp registers
///
/// ```
/// use riscv_pac::result::{Error, Result};
///
/// /// HART IDs for the target CLINT peripheral
/// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// pub enum HartId { H0 = 0, H1 = 1, H2 = 2 }
///
/// // Implement `HartIdNumber` for `HartId`
/// unsafe impl riscv_peripheral::aclint::HartIdNumber for HartId {
///   const MAX_HART_ID_NUMBER: usize = Self::H2 as usize;
///   fn number(self) -> usize { self as _ }
///   fn from_number(number: usize) -> Result<Self> {
///     match number {
///      0 => Ok(HartId::H0),
///      1 => Ok(HartId::H1),
///      2 => Ok(HartId::H2),
///      _ => Err(Error::InvalidVariant(number)),
///     }
///   }
/// }
///
/// riscv_peripheral::clint_codegen!(
///     base 0x0200_0000,
///     mtimecmps [mtimecmp0 = (HartId::H0, "`H0`"), mtimecmp1 = (HartId::H1, "`H1`"), mtimecmp2 = (HartId::H2, "`H2`")],
///     msips [msip0=(HartId::H0,"`H0`"), msip1=(HartId::H1,"`H1`"), msip2=(HartId::H2,"`H2`")], // do not forget the ending comma!
/// );
///
/// let mswi = CLINT::mswi(); // MSWI peripheral
/// let mtimer = CLINT::mtimer(); // MTIMER peripheral
///
/// let mtimecmp0 = CLINT::mtimecmp0(); // mtimecmp register for HART 0
/// let mtimecmp1 = CLINT::mtimecmp1(); // mtimecmp register for HART 1
/// let mtimecmp2 = CLINT::mtimecmp2(); // mtimecmp register for HART 2
///
/// let msip0 = CLINT::msip0(); // msip register for HART 0
/// let msip1 = CLINT::msip1(); // msip register for HART 1
/// let msip2 = CLINT::msip2(); // msip register for HART 2
/// ```
#[macro_export]
macro_rules! clint_codegen {
    () => {
        #[allow(unused_imports)]
        use CLINT as _; // assert that the CLINT struct is defined
    };
    (base $addr:literal, $($tail:tt)*) => {
        /// CLINT peripheral
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct CLINT;

        unsafe impl $crate::aclint::Clint for CLINT {
            const BASE: usize = $addr;
        }

        impl CLINT {
            /// Returns `true` if a machine timer **OR** software interrupt is pending.
            #[inline]
            pub fn is_interrupting() -> bool {
                Self::mswi_is_interrupting() || Self::mtimer_is_interrupting()
            }

            /// Returns `true` if machine timer **OR** software interrupts are enabled.
            pub fn is_enabled() -> bool {
                Self::mswi_is_enabled() || Self::mtimer_is_enabled()
            }

            /// Enables machine timer **AND** software interrupts to allow the CLINT to trigger interrupts.
            ///
            /// # Safety
            ///
            /// Enabling the `CLINT` may break mask-based critical sections.
            #[inline]
            pub unsafe fn enable() {
                Self::mswi_enable();
                Self::mtimer_enable();
            }

            /// Disables machine timer **AND** software interrupts to prevent the CLINT from triggering interrupts.
            #[inline]
            pub fn disable() {
                Self::mswi_disable();
                Self::mtimer_disable();
            }

            /// Returns `true` if a machine software interrupt is pending.
            #[inline]
            pub fn mswi_is_interrupting() -> bool {
                $crate::riscv::register::mip::read().msoft()
            }

            /// Returns `true` if Machine Software Interrupts are enabled.
            #[inline]
            pub fn mswi_is_enabled() -> bool {
                $crate::riscv::register::mie::read().msoft()
            }

            /// Enables the `MSWI` peripheral.
            ///
            /// # Safety
            ///
            /// Enabling the `MSWI` may break mask-based critical sections.
            #[inline]
            pub unsafe fn mswi_enable() {
                $crate::riscv::register::mie::set_msoft();
            }

            /// Disables the `MSWI` peripheral.
            #[inline]
            pub fn mswi_disable() {
                // SAFETY: it is safe to disable interrupts
                unsafe { $crate::riscv::register::mie::clear_msoft() };
            }

            /// Returns the `MSWI` peripheral.
            #[inline]
            pub const fn mswi() -> $crate::aclint::mswi::MSWI {
                $crate::aclint::CLINT::<CLINT>::mswi()
            }

            /// Returns `true` if a machine timer interrupt is pending.
            #[inline]
            pub fn mtimer_is_interrupting() -> bool {
                $crate::riscv::register::mip::read().mtimer()
            }

            /// Returns `true` if Machine Timer Interrupts are enabled.
            #[inline]
            pub fn mtimer_is_enabled() -> bool {
                $crate::riscv::register::mie::read().mtimer()
            }

            /// Sets the Machine Timer Interrupt bit of the `mie` CSR.
            /// This bit must be set for the `MTIMER` to trigger machine timer interrupts.
            ///
            /// # Safety
            ///
            /// Enabling the `MTIMER` may break mask-based critical sections.
            #[inline]
            pub unsafe fn mtimer_enable() {
                $crate::riscv::register::mie::set_mtimer();
            }

            /// Clears the Machine Timer Interrupt bit of the `mie` CSR.
            /// When cleared, the `MTIMER` cannot trigger machine timer interrupts.
            #[inline]
            pub fn mtimer_disable() {
                // SAFETY: it is safe to disable interrupts
                unsafe { $crate::riscv::register::mie::clear_mtimer() };
            }

            /// Returns the `MTIMER` peripheral.
            #[inline]
            pub const fn mtimer() -> $crate::aclint::mtimer::MTIMER {
                $crate::aclint::CLINT::<CLINT>::mtimer()
            }

            /// Returns the `MTIME` register of the `MTIMER` peripheral.
            #[inline]
            pub const fn mtime() -> $crate::aclint::mtimer::MTIME {
                Self::mtimer().mtime
            }
        }
        $crate::clint_codegen!($($tail)*);
    };
    (freq $freq:literal, $($tail:tt)*) => {
        impl CLINT {
            /// Returns the frequency of the `MTIME` register.
            #[inline]
            pub const fn freq() -> usize {
                $freq
            }

            /// Delay implementation for CLINT peripherals.
            ///
            /// # Note
            ///
            /// You must export the [`embedded_hal::delay::DelayNs`] trait in order to use delay methods.
            #[inline]
            pub const fn delay() -> $crate::hal::aclint::Delay {
                $crate::hal::aclint::Delay::new(Self::mtime(), Self::freq())
            }
        }
        $crate::clint_codegen!($($tail)*);
    };
    (async_delay, $($tail:tt)*) => {
        impl CLINT {
            /// Asynchronous delay implementation for CLINT peripherals.
            ///
            /// # Note
            ///
            /// You must export the [`embedded_hal_async::delay::DelayNs`] trait in order to use delay methods.
            ///
            /// This implementation relies on the machine-level timer interrupts to wake futures.
            /// Therefore, it needs to schedule the machine-level timer interrupts via the `MTIMECMP` register assigned to the current HART.
            /// Thus, the `Delay` instance must be created on the same HART that is used to call the asynchronous delay methods.
            /// Additionally, the rest of the application must not modify the `MTIMER` register assigned to the current HART.
            #[inline]
            pub fn async_delay() -> $crate::hal_async::aclint::Delay {
                $crate::hal_async::aclint::Delay::new(Self::freq())
            }
        }
        $crate::clint_codegen!($($tail)*);
    };
    (msips [$($fn:ident = ($hart:expr , $shart:expr)),+], $($tail:tt)*) => {
        impl CLINT {
            $(
                #[doc = "Returns the `msip` register for HART "]
                #[doc = $shart]
                #[doc = "."]
                #[inline]
                pub fn $fn() -> $crate::aclint::mswi::MSIP {
                    Self::mswi().msip($hart)
                }
            )*
        }
        $crate::clint_codegen!($($tail)*);
    };
    (mtimecmps [$($fn:ident = ($hart:expr , $shart:expr)),+], $($tail:tt)*) => {
        impl CLINT {
            $(
                #[doc = "Returns the `mtimecmp` register for HART "]
                #[doc = $shart]
                #[doc = "."]
                #[inline]
                pub fn $fn() -> $crate::aclint::mtimer::MTIMECMP {
                    Self::mtimer().mtimecmp($hart)
                }
            )*
        }
        $crate::clint_codegen!($($tail)*);
    };
}

/// Macro to create interfaces to PLIC peripherals in PACs.
/// The resulting struct will be named `PLIC`, and will provide safe access to the PLIC registers.
///
/// This macro expects 2 different argument types:
///
/// - Base address (**MANDATORY**): base address of the PLIC peripheral of the target.
/// - Per-HART contexts (**OPTIONAL**): a list of `ctx` contexts for easing access to per-HART PLIC contexts.
///
/// Check the examples below for more details about the usage and syntax of this macro.
///
/// # Example
///
/// ## Base address only
///
/// ```
/// use riscv_peripheral::clint_codegen;
///
/// riscv_peripheral::plic_codegen!(base 0x0C00_0000,); // do not forget the ending comma!
///
/// let priorities = PLIC::priorities(); // Priorities registers
/// let pendings = PLIC::pendings();     // Pendings registers
/// ```
#[macro_export]
macro_rules! plic_codegen {
    () => {
        #[allow(unused_imports)]
        use PLIC as _; // assert that the PLIC struct is defined
    };
    (base $addr:literal, $($tail:tt)*) => {
        /// PLIC peripheral
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct PLIC;

        unsafe impl $crate::plic::Plic for PLIC {
            const BASE: usize = $addr;
        }

        impl PLIC {
            /// Returns `true` if a machine external interrupt is pending.
            #[inline]
            pub fn is_interrupting() -> bool {
                $crate::riscv::register::mip::read().mext()
            }

            /// Returns true if Machine External Interrupts are enabled.
            #[inline]
            pub fn is_enabled() -> bool {
                $crate::riscv::register::mie::read().mext()
            }

            /// Enables machine external interrupts to allow the PLIC to trigger interrupts.
            ///
            /// # Safety
            ///
            /// Enabling the `PLIC` may break mask-based critical sections.
            #[inline]
            pub unsafe fn enable() {
                $crate::riscv::register::mie::set_mext();
            }

            /// Disables machine external interrupts to prevent the PLIC from triggering interrupts.
            #[inline]
            pub fn disable() {
                // SAFETY: it is safe to disable interrupts
                unsafe { $crate::riscv::register::mie::clear_mext() };
            }

            /// Returns the priorities register of the PLIC.
            #[inline]
            pub fn priorities() -> $crate::plic::priorities::PRIORITIES {
                $crate::plic::PLIC::<PLIC>::priorities()
            }

            /// Returns the pendings register of the PLIC.
            #[inline]
            pub fn pendings() -> $crate::plic::pendings::PENDINGS {
                $crate::plic::PLIC::<PLIC>::pendings()
            }

            /// Returns the context proxy of a given PLIC HART context.
            #[inline]
            pub fn ctx<H: $crate::plic::HartIdNumber>(hart_id: H) -> $crate::plic::CTX<Self> {
                $crate::plic::PLIC::<PLIC>::ctx(hart_id)
            }

            /// Returns the PLIC HART context for the current HART.
            ///
            /// # Note
            ///
            /// This function determines the current HART ID by reading the [`riscv::register::mhartid`] CSR.
            /// Thus, it can only be used in M-mode. For S-mode, use [`PLIC::ctx`] instead.
            #[inline]
            pub fn ctx_mhartid() -> $crate::plic::CTX<Self> {
                $crate::plic::PLIC::<PLIC>::ctx_mhartid()
            }
        }
        $crate::plic_codegen!($($tail)*);
    };
    (ctxs [$($fn:ident = ($ctx:expr , $sctx:expr)),+], $($tail:tt)*) => {
        impl PLIC {
            $(
                #[doc = "Returns a PLIC context proxy for context of HART "]
                #[doc = $sctx]
                #[doc = "."]
                #[inline]
                pub fn $fn() -> $crate::plic::CTX<Self> {
                    Self::ctx($ctx)
                }
            )*
        }
        $crate::plic_codegen!($($tail)*);
    };
}
