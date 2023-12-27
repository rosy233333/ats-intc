#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    control: CONTROL,
    stask_lsb: STASK_LSB,
    stask_msb: STASK_MSB,
    ftask_lsb: FTASK_LSB,
    ftask_msb: FTASK_MSB,
    handler_lsb: HANDLER_LSB,
    handler_msb: HANDLER_MSB,
    irq: IRQ,
    memcache_lsb: MEMCACHE_LSB,
    memcache_msb: MEMCACHE_MSB,
    priority: PRIORITY,
}
impl RegisterBlock {
    #[doc = "0x00 - The transaction control register"]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x04 - When the software store the task, it will write the register.(LSB)"]
    #[inline(always)]
    pub const fn stask_lsb(&self) -> &STASK_LSB {
        &self.stask_lsb
    }
    #[doc = "0x08 - When the software store the task, it will write the register.(MSB)"]
    #[inline(always)]
    pub const fn stask_msb(&self) -> &STASK_MSB {
        &self.stask_msb
    }
    #[doc = "0x0c - When the software fetch task, it will read from this register.(LSB)"]
    #[inline(always)]
    pub const fn ftask_lsb(&self) -> &FTASK_LSB {
        &self.ftask_lsb
    }
    #[doc = "0x10 - When the software fetch task, it will read from this register.(MSB)"]
    #[inline(always)]
    pub const fn ftask_msb(&self) -> &FTASK_MSB {
        &self.ftask_msb
    }
    #[doc = "0x14 - The handler task pointer lsb"]
    #[inline(always)]
    pub const fn handler_lsb(&self) -> &HANDLER_LSB {
        &self.handler_lsb
    }
    #[doc = "0x18 - The handler task pointer msb"]
    #[inline(always)]
    pub const fn handler_msb(&self) -> &HANDLER_MSB {
        &self.handler_msb
    }
    #[doc = "0x1c - The irq register, when the software store handler task, it will write the relative irq."]
    #[inline(always)]
    pub const fn irq(&self) -> &IRQ {
        &self.irq
    }
    #[doc = "0x20 - The in-memory cache address.(LSB)"]
    #[inline(always)]
    pub const fn memcache_lsb(&self) -> &MEMCACHE_LSB {
        &self.memcache_lsb
    }
    #[doc = "0x24 - The in-memory cache address.(MSB)"]
    #[inline(always)]
    pub const fn memcache_msb(&self) -> &MEMCACHE_MSB {
        &self.memcache_msb
    }
    #[doc = "0x28 - The task priority."]
    #[inline(always)]
    pub const fn priority(&self) -> &PRIORITY {
        &self.priority
    }
}
#[doc = "control (rw) register accessor: The transaction control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "The transaction control register"]
pub mod control;
#[doc = "stask_lsb (rw) register accessor: When the software store the task, it will write the register.(LSB)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stask_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stask_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stask_lsb`]
module"]
pub type STASK_LSB = crate::Reg<stask_lsb::STASK_LSB_SPEC>;
#[doc = "When the software store the task, it will write the register.(LSB)"]
pub mod stask_lsb;
#[doc = "stask_msb (rw) register accessor: When the software store the task, it will write the register.(MSB)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stask_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stask_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stask_msb`]
module"]
pub type STASK_MSB = crate::Reg<stask_msb::STASK_MSB_SPEC>;
#[doc = "When the software store the task, it will write the register.(MSB)"]
pub mod stask_msb;
#[doc = "ftask_lsb (rw) register accessor: When the software fetch task, it will read from this register.(LSB)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftask_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftask_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftask_lsb`]
module"]
pub type FTASK_LSB = crate::Reg<ftask_lsb::FTASK_LSB_SPEC>;
#[doc = "When the software fetch task, it will read from this register.(LSB)"]
pub mod ftask_lsb;
#[doc = "ftask_msb (rw) register accessor: When the software fetch task, it will read from this register.(MSB)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftask_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftask_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftask_msb`]
module"]
pub type FTASK_MSB = crate::Reg<ftask_msb::FTASK_MSB_SPEC>;
#[doc = "When the software fetch task, it will read from this register.(MSB)"]
pub mod ftask_msb;
#[doc = "handler_lsb (rw) register accessor: The handler task pointer lsb\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`handler_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`handler_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@handler_lsb`]
module"]
pub type HANDLER_LSB = crate::Reg<handler_lsb::HANDLER_LSB_SPEC>;
#[doc = "The handler task pointer lsb"]
pub mod handler_lsb;
#[doc = "handler_msb (rw) register accessor: The handler task pointer msb\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`handler_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`handler_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@handler_msb`]
module"]
pub type HANDLER_MSB = crate::Reg<handler_msb::HANDLER_MSB_SPEC>;
#[doc = "The handler task pointer msb"]
pub mod handler_msb;
#[doc = "irq (rw) register accessor: The irq register, when the software store handler task, it will write the relative irq.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq`]
module"]
pub type IRQ = crate::Reg<irq::IRQ_SPEC>;
#[doc = "The irq register, when the software store handler task, it will write the relative irq."]
pub mod irq;
#[doc = "memcache_lsb (rw) register accessor: The in-memory cache address.(LSB)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memcache_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memcache_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memcache_lsb`]
module"]
pub type MEMCACHE_LSB = crate::Reg<memcache_lsb::MEMCACHE_LSB_SPEC>;
#[doc = "The in-memory cache address.(LSB)"]
pub mod memcache_lsb;
#[doc = "memcache_Msb (rw) register accessor: The in-memory cache address.(MSB)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memcache_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memcache_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memcache_msb`]
module"]
pub type MEMCACHE_MSB = crate::Reg<memcache_msb::MEMCACHE_MSB_SPEC>;
#[doc = "The in-memory cache address.(MSB)"]
pub mod memcache_msb;
#[doc = "priority (rw) register accessor: The task priority.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority`]
module"]
pub type PRIORITY = crate::Reg<priority::PRIORITY_SPEC>;
#[doc = "The task priority."]
pub mod priority;
