#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    priority0: Priority0,
    priority1: Priority1,
    priority2: Priority2,
    priority3: Priority3,
    priority4: Priority4,
    priority5: Priority5,
    priority6: Priority6,
    priority7: Priority7,
    priority8: Priority8,
    priority9: Priority9,
    priority10: Priority10,
    priority11: Priority11,
    priority12: Priority12,
    priority13: Priority13,
    priority14: Priority14,
    priority15: Priority15,
    priority16: Priority16,
    priority17: Priority17,
    priority18: Priority18,
    priority19: Priority19,
    priority20: Priority20,
    priority21: Priority21,
    priority22: Priority22,
    priority23: Priority23,
    priority24: Priority24,
    priority25: Priority25,
    priority26: Priority26,
    priority27: Priority27,
    priority28: Priority28,
    priority29: Priority29,
    priority30: Priority30,
    priority31: Priority31,
    priority32: Priority32,
    priority33: Priority33,
    priority34: Priority34,
    priority35: Priority35,
    priority36: Priority36,
    priority37: Priority37,
    priority38: Priority38,
    priority39: Priority39,
    priority40: Priority40,
    priority41: Priority41,
    priority42: Priority42,
    priority43: Priority43,
    priority44: Priority44,
    priority45: Priority45,
    priority46: Priority46,
    priority47: Priority47,
    priority48: Priority48,
    priority49: Priority49,
    priority50: Priority50,
    priority51: Priority51,
    priority52: Priority52,
    priority53: Priority53,
    priority54: Priority54,
    priority55: Priority55,
    priority56: Priority56,
    priority57: Priority57,
    priority58: Priority58,
    priority59: Priority59,
    priority60: Priority60,
    priority61: Priority61,
    priority62: Priority62,
    priority63: Priority63,
    _reserved64: [u8; 0x0f00],
    pending_0_32: Pending0_32,
    pending_33_63: Pending33_63,
    _reserved66: [u8; 0x0ff8],
    intr_en_0_32: IntrEn0_32,
    intr_en_33_63: IntrEn33_63,
    _reserved68: [u8; 0xdff8],
    priority_thres: PriorityThres,
    intr_complete: IntrComplete,
}
impl RegisterBlock {
    #[doc = "0x00 - Priority register for interrupt source 0"]
    #[inline(always)]
    pub const fn priority0(&self) -> &Priority0 {
        &self.priority0
    }
    #[doc = "0x04 - Priority register for interrupt source 1"]
    #[inline(always)]
    pub const fn priority1(&self) -> &Priority1 {
        &self.priority1
    }
    #[doc = "0x08 - Priority register for interrupt source 2"]
    #[inline(always)]
    pub const fn priority2(&self) -> &Priority2 {
        &self.priority2
    }
    #[doc = "0x0c - Priority register for interrupt source 3"]
    #[inline(always)]
    pub const fn priority3(&self) -> &Priority3 {
        &self.priority3
    }
    #[doc = "0x10 - Priority register for interrupt source 4"]
    #[inline(always)]
    pub const fn priority4(&self) -> &Priority4 {
        &self.priority4
    }
    #[doc = "0x14 - Priority register for interrupt source 5"]
    #[inline(always)]
    pub const fn priority5(&self) -> &Priority5 {
        &self.priority5
    }
    #[doc = "0x18 - Priority register for interrupt source 6"]
    #[inline(always)]
    pub const fn priority6(&self) -> &Priority6 {
        &self.priority6
    }
    #[doc = "0x1c - Priority register for interrupt source 7"]
    #[inline(always)]
    pub const fn priority7(&self) -> &Priority7 {
        &self.priority7
    }
    #[doc = "0x20 - Priority register for interrupt source 8"]
    #[inline(always)]
    pub const fn priority8(&self) -> &Priority8 {
        &self.priority8
    }
    #[doc = "0x24 - Priority register for interrupt source 9"]
    #[inline(always)]
    pub const fn priority9(&self) -> &Priority9 {
        &self.priority9
    }
    #[doc = "0x28 - Priority register for interrupt source 10"]
    #[inline(always)]
    pub const fn priority10(&self) -> &Priority10 {
        &self.priority10
    }
    #[doc = "0x2c - Priority register for interrupt source 11"]
    #[inline(always)]
    pub const fn priority11(&self) -> &Priority11 {
        &self.priority11
    }
    #[doc = "0x30 - Priority register for interrupt source 12"]
    #[inline(always)]
    pub const fn priority12(&self) -> &Priority12 {
        &self.priority12
    }
    #[doc = "0x34 - Priority register for interrupt source 13"]
    #[inline(always)]
    pub const fn priority13(&self) -> &Priority13 {
        &self.priority13
    }
    #[doc = "0x38 - Priority register for interrupt source 14"]
    #[inline(always)]
    pub const fn priority14(&self) -> &Priority14 {
        &self.priority14
    }
    #[doc = "0x3c - Priority register for interrupt source 15"]
    #[inline(always)]
    pub const fn priority15(&self) -> &Priority15 {
        &self.priority15
    }
    #[doc = "0x40 - Priority register for interrupt source 16"]
    #[inline(always)]
    pub const fn priority16(&self) -> &Priority16 {
        &self.priority16
    }
    #[doc = "0x44 - Priority register for interrupt source 17"]
    #[inline(always)]
    pub const fn priority17(&self) -> &Priority17 {
        &self.priority17
    }
    #[doc = "0x48 - Priority register for interrupt source 18"]
    #[inline(always)]
    pub const fn priority18(&self) -> &Priority18 {
        &self.priority18
    }
    #[doc = "0x4c - Priority register for interrupt source 19"]
    #[inline(always)]
    pub const fn priority19(&self) -> &Priority19 {
        &self.priority19
    }
    #[doc = "0x50 - Priority register for interrupt source 20"]
    #[inline(always)]
    pub const fn priority20(&self) -> &Priority20 {
        &self.priority20
    }
    #[doc = "0x54 - Priority register for interrupt source 21"]
    #[inline(always)]
    pub const fn priority21(&self) -> &Priority21 {
        &self.priority21
    }
    #[doc = "0x58 - Priority register for interrupt source 22"]
    #[inline(always)]
    pub const fn priority22(&self) -> &Priority22 {
        &self.priority22
    }
    #[doc = "0x5c - Priority register for interrupt source 23"]
    #[inline(always)]
    pub const fn priority23(&self) -> &Priority23 {
        &self.priority23
    }
    #[doc = "0x60 - Priority register for interrupt source 24"]
    #[inline(always)]
    pub const fn priority24(&self) -> &Priority24 {
        &self.priority24
    }
    #[doc = "0x64 - Priority register for interrupt source 25"]
    #[inline(always)]
    pub const fn priority25(&self) -> &Priority25 {
        &self.priority25
    }
    #[doc = "0x68 - Priority register for interrupt source 26"]
    #[inline(always)]
    pub const fn priority26(&self) -> &Priority26 {
        &self.priority26
    }
    #[doc = "0x6c - Priority register for interrupt source 27"]
    #[inline(always)]
    pub const fn priority27(&self) -> &Priority27 {
        &self.priority27
    }
    #[doc = "0x70 - Priority register for interrupt source 28"]
    #[inline(always)]
    pub const fn priority28(&self) -> &Priority28 {
        &self.priority28
    }
    #[doc = "0x74 - Priority register for interrupt source 29"]
    #[inline(always)]
    pub const fn priority29(&self) -> &Priority29 {
        &self.priority29
    }
    #[doc = "0x78 - Priority register for interrupt source 30"]
    #[inline(always)]
    pub const fn priority30(&self) -> &Priority30 {
        &self.priority30
    }
    #[doc = "0x7c - Priority register for interrupt source 31"]
    #[inline(always)]
    pub const fn priority31(&self) -> &Priority31 {
        &self.priority31
    }
    #[doc = "0x80 - Priority register for interrupt source 32"]
    #[inline(always)]
    pub const fn priority32(&self) -> &Priority32 {
        &self.priority32
    }
    #[doc = "0x84 - Priority register for interrupt source 33"]
    #[inline(always)]
    pub const fn priority33(&self) -> &Priority33 {
        &self.priority33
    }
    #[doc = "0x88 - Priority register for interrupt source 34"]
    #[inline(always)]
    pub const fn priority34(&self) -> &Priority34 {
        &self.priority34
    }
    #[doc = "0x8c - Priority register for interrupt source 35"]
    #[inline(always)]
    pub const fn priority35(&self) -> &Priority35 {
        &self.priority35
    }
    #[doc = "0x90 - Priority register for interrupt source 36"]
    #[inline(always)]
    pub const fn priority36(&self) -> &Priority36 {
        &self.priority36
    }
    #[doc = "0x94 - Priority register for interrupt source 37"]
    #[inline(always)]
    pub const fn priority37(&self) -> &Priority37 {
        &self.priority37
    }
    #[doc = "0x98 - Priority register for interrupt source 38"]
    #[inline(always)]
    pub const fn priority38(&self) -> &Priority38 {
        &self.priority38
    }
    #[doc = "0x9c - Priority register for interrupt source 39"]
    #[inline(always)]
    pub const fn priority39(&self) -> &Priority39 {
        &self.priority39
    }
    #[doc = "0xa0 - Priority register for interrupt source 40"]
    #[inline(always)]
    pub const fn priority40(&self) -> &Priority40 {
        &self.priority40
    }
    #[doc = "0xa4 - Priority register for interrupt source 41"]
    #[inline(always)]
    pub const fn priority41(&self) -> &Priority41 {
        &self.priority41
    }
    #[doc = "0xa8 - Priority register for interrupt source 42"]
    #[inline(always)]
    pub const fn priority42(&self) -> &Priority42 {
        &self.priority42
    }
    #[doc = "0xac - Priority register for interrupt source 43"]
    #[inline(always)]
    pub const fn priority43(&self) -> &Priority43 {
        &self.priority43
    }
    #[doc = "0xb0 - Priority register for interrupt source 44"]
    #[inline(always)]
    pub const fn priority44(&self) -> &Priority44 {
        &self.priority44
    }
    #[doc = "0xb4 - Priority register for interrupt source 45"]
    #[inline(always)]
    pub const fn priority45(&self) -> &Priority45 {
        &self.priority45
    }
    #[doc = "0xb8 - Priority register for interrupt source 46"]
    #[inline(always)]
    pub const fn priority46(&self) -> &Priority46 {
        &self.priority46
    }
    #[doc = "0xbc - Priority register for interrupt source 47"]
    #[inline(always)]
    pub const fn priority47(&self) -> &Priority47 {
        &self.priority47
    }
    #[doc = "0xc0 - Priority register for interrupt source 48"]
    #[inline(always)]
    pub const fn priority48(&self) -> &Priority48 {
        &self.priority48
    }
    #[doc = "0xc4 - Priority register for interrupt source 49"]
    #[inline(always)]
    pub const fn priority49(&self) -> &Priority49 {
        &self.priority49
    }
    #[doc = "0xc8 - Priority register for interrupt source 50"]
    #[inline(always)]
    pub const fn priority50(&self) -> &Priority50 {
        &self.priority50
    }
    #[doc = "0xcc - Priority register for interrupt source 51"]
    #[inline(always)]
    pub const fn priority51(&self) -> &Priority51 {
        &self.priority51
    }
    #[doc = "0xd0 - Priority register for interrupt source 52"]
    #[inline(always)]
    pub const fn priority52(&self) -> &Priority52 {
        &self.priority52
    }
    #[doc = "0xd4 - Priority register for interrupt source 53"]
    #[inline(always)]
    pub const fn priority53(&self) -> &Priority53 {
        &self.priority53
    }
    #[doc = "0xd8 - Priority register for interrupt source 54"]
    #[inline(always)]
    pub const fn priority54(&self) -> &Priority54 {
        &self.priority54
    }
    #[doc = "0xdc - Priority register for interrupt source 55"]
    #[inline(always)]
    pub const fn priority55(&self) -> &Priority55 {
        &self.priority55
    }
    #[doc = "0xe0 - Priority register for interrupt source 56"]
    #[inline(always)]
    pub const fn priority56(&self) -> &Priority56 {
        &self.priority56
    }
    #[doc = "0xe4 - Priority register for interrupt source 57"]
    #[inline(always)]
    pub const fn priority57(&self) -> &Priority57 {
        &self.priority57
    }
    #[doc = "0xe8 - Priority register for interrupt source 58"]
    #[inline(always)]
    pub const fn priority58(&self) -> &Priority58 {
        &self.priority58
    }
    #[doc = "0xec - Priority register for interrupt source 59"]
    #[inline(always)]
    pub const fn priority59(&self) -> &Priority59 {
        &self.priority59
    }
    #[doc = "0xf0 - Priority register for interrupt source 60"]
    #[inline(always)]
    pub const fn priority60(&self) -> &Priority60 {
        &self.priority60
    }
    #[doc = "0xf4 - Priority register for interrupt source 61"]
    #[inline(always)]
    pub const fn priority61(&self) -> &Priority61 {
        &self.priority61
    }
    #[doc = "0xf8 - Priority register for interrupt source 62"]
    #[inline(always)]
    pub const fn priority62(&self) -> &Priority62 {
        &self.priority62
    }
    #[doc = "0xfc - Priority register for interrupt source 63"]
    #[inline(always)]
    pub const fn priority63(&self) -> &Priority63 {
        &self.priority63
    }
    #[doc = "0x1000 - Interrupt pending bits of sources 0-32"]
    #[inline(always)]
    pub const fn pending_0_32(&self) -> &Pending0_32 {
        &self.pending_0_32
    }
    #[doc = "0x1004 - Interrupt pending bits of sources 33-63"]
    #[inline(always)]
    pub const fn pending_33_63(&self) -> &Pending33_63 {
        &self.pending_33_63
    }
    #[doc = "0x2000 - Interrupt enable bits of sources 0-32"]
    #[inline(always)]
    pub const fn intr_en_0_32(&self) -> &IntrEn0_32 {
        &self.intr_en_0_32
    }
    #[doc = "0x2004 - Interrupt enable bits of sources 33-63"]
    #[inline(always)]
    pub const fn intr_en_33_63(&self) -> &IntrEn33_63 {
        &self.intr_en_33_63
    }
    #[doc = "0x10000 - Priority threshold register"]
    #[inline(always)]
    pub const fn priority_thres(&self) -> &PriorityThres {
        &self.priority_thres
    }
    #[doc = "0x10004 - Interrupt claim/complete register"]
    #[inline(always)]
    pub const fn intr_complete(&self) -> &IntrComplete {
        &self.intr_complete
    }
}
#[doc = "PRIORITY0 (rw) register accessor: Priority register for interrupt source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`priority0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority0`]
module"]
#[doc(alias = "PRIORITY0")]
pub type Priority0 = crate::Reg<priority0::Priority0Spec>;
#[doc = "Priority register for interrupt source 0"]
pub mod priority0;
#[doc = "PRIORITY1 (rw) register accessor: Priority register for interrupt source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`priority1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority1`]
module"]
#[doc(alias = "PRIORITY1")]
pub type Priority1 = crate::Reg<priority1::Priority1Spec>;
#[doc = "Priority register for interrupt source 1"]
pub mod priority1;
#[doc = "PRIORITY2 (rw) register accessor: Priority register for interrupt source 2\n\nYou can [`read`](crate::Reg::read) this register and get [`priority2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority2`]
module"]
#[doc(alias = "PRIORITY2")]
pub type Priority2 = crate::Reg<priority2::Priority2Spec>;
#[doc = "Priority register for interrupt source 2"]
pub mod priority2;
#[doc = "PRIORITY3 (rw) register accessor: Priority register for interrupt source 3\n\nYou can [`read`](crate::Reg::read) this register and get [`priority3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority3`]
module"]
#[doc(alias = "PRIORITY3")]
pub type Priority3 = crate::Reg<priority3::Priority3Spec>;
#[doc = "Priority register for interrupt source 3"]
pub mod priority3;
#[doc = "PRIORITY4 (rw) register accessor: Priority register for interrupt source 4\n\nYou can [`read`](crate::Reg::read) this register and get [`priority4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority4`]
module"]
#[doc(alias = "PRIORITY4")]
pub type Priority4 = crate::Reg<priority4::Priority4Spec>;
#[doc = "Priority register for interrupt source 4"]
pub mod priority4;
#[doc = "PRIORITY5 (rw) register accessor: Priority register for interrupt source 5\n\nYou can [`read`](crate::Reg::read) this register and get [`priority5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority5`]
module"]
#[doc(alias = "PRIORITY5")]
pub type Priority5 = crate::Reg<priority5::Priority5Spec>;
#[doc = "Priority register for interrupt source 5"]
pub mod priority5;
#[doc = "PRIORITY6 (rw) register accessor: Priority register for interrupt source 6\n\nYou can [`read`](crate::Reg::read) this register and get [`priority6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority6`]
module"]
#[doc(alias = "PRIORITY6")]
pub type Priority6 = crate::Reg<priority6::Priority6Spec>;
#[doc = "Priority register for interrupt source 6"]
pub mod priority6;
#[doc = "PRIORITY7 (rw) register accessor: Priority register for interrupt source 7\n\nYou can [`read`](crate::Reg::read) this register and get [`priority7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority7`]
module"]
#[doc(alias = "PRIORITY7")]
pub type Priority7 = crate::Reg<priority7::Priority7Spec>;
#[doc = "Priority register for interrupt source 7"]
pub mod priority7;
#[doc = "PRIORITY8 (rw) register accessor: Priority register for interrupt source 8\n\nYou can [`read`](crate::Reg::read) this register and get [`priority8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority8`]
module"]
#[doc(alias = "PRIORITY8")]
pub type Priority8 = crate::Reg<priority8::Priority8Spec>;
#[doc = "Priority register for interrupt source 8"]
pub mod priority8;
#[doc = "PRIORITY9 (rw) register accessor: Priority register for interrupt source 9\n\nYou can [`read`](crate::Reg::read) this register and get [`priority9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority9`]
module"]
#[doc(alias = "PRIORITY9")]
pub type Priority9 = crate::Reg<priority9::Priority9Spec>;
#[doc = "Priority register for interrupt source 9"]
pub mod priority9;
#[doc = "PRIORITY10 (rw) register accessor: Priority register for interrupt source 10\n\nYou can [`read`](crate::Reg::read) this register and get [`priority10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority10`]
module"]
#[doc(alias = "PRIORITY10")]
pub type Priority10 = crate::Reg<priority10::Priority10Spec>;
#[doc = "Priority register for interrupt source 10"]
pub mod priority10;
#[doc = "PRIORITY11 (rw) register accessor: Priority register for interrupt source 11\n\nYou can [`read`](crate::Reg::read) this register and get [`priority11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority11`]
module"]
#[doc(alias = "PRIORITY11")]
pub type Priority11 = crate::Reg<priority11::Priority11Spec>;
#[doc = "Priority register for interrupt source 11"]
pub mod priority11;
#[doc = "PRIORITY12 (rw) register accessor: Priority register for interrupt source 12\n\nYou can [`read`](crate::Reg::read) this register and get [`priority12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority12`]
module"]
#[doc(alias = "PRIORITY12")]
pub type Priority12 = crate::Reg<priority12::Priority12Spec>;
#[doc = "Priority register for interrupt source 12"]
pub mod priority12;
#[doc = "PRIORITY13 (rw) register accessor: Priority register for interrupt source 13\n\nYou can [`read`](crate::Reg::read) this register and get [`priority13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority13`]
module"]
#[doc(alias = "PRIORITY13")]
pub type Priority13 = crate::Reg<priority13::Priority13Spec>;
#[doc = "Priority register for interrupt source 13"]
pub mod priority13;
#[doc = "PRIORITY14 (rw) register accessor: Priority register for interrupt source 14\n\nYou can [`read`](crate::Reg::read) this register and get [`priority14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority14`]
module"]
#[doc(alias = "PRIORITY14")]
pub type Priority14 = crate::Reg<priority14::Priority14Spec>;
#[doc = "Priority register for interrupt source 14"]
pub mod priority14;
#[doc = "PRIORITY15 (rw) register accessor: Priority register for interrupt source 15\n\nYou can [`read`](crate::Reg::read) this register and get [`priority15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority15`]
module"]
#[doc(alias = "PRIORITY15")]
pub type Priority15 = crate::Reg<priority15::Priority15Spec>;
#[doc = "Priority register for interrupt source 15"]
pub mod priority15;
#[doc = "PRIORITY16 (rw) register accessor: Priority register for interrupt source 16\n\nYou can [`read`](crate::Reg::read) this register and get [`priority16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority16`]
module"]
#[doc(alias = "PRIORITY16")]
pub type Priority16 = crate::Reg<priority16::Priority16Spec>;
#[doc = "Priority register for interrupt source 16"]
pub mod priority16;
#[doc = "PRIORITY17 (rw) register accessor: Priority register for interrupt source 17\n\nYou can [`read`](crate::Reg::read) this register and get [`priority17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority17`]
module"]
#[doc(alias = "PRIORITY17")]
pub type Priority17 = crate::Reg<priority17::Priority17Spec>;
#[doc = "Priority register for interrupt source 17"]
pub mod priority17;
#[doc = "PRIORITY18 (rw) register accessor: Priority register for interrupt source 18\n\nYou can [`read`](crate::Reg::read) this register and get [`priority18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority18`]
module"]
#[doc(alias = "PRIORITY18")]
pub type Priority18 = crate::Reg<priority18::Priority18Spec>;
#[doc = "Priority register for interrupt source 18"]
pub mod priority18;
#[doc = "PRIORITY19 (rw) register accessor: Priority register for interrupt source 19\n\nYou can [`read`](crate::Reg::read) this register and get [`priority19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority19`]
module"]
#[doc(alias = "PRIORITY19")]
pub type Priority19 = crate::Reg<priority19::Priority19Spec>;
#[doc = "Priority register for interrupt source 19"]
pub mod priority19;
#[doc = "PRIORITY20 (rw) register accessor: Priority register for interrupt source 20\n\nYou can [`read`](crate::Reg::read) this register and get [`priority20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority20`]
module"]
#[doc(alias = "PRIORITY20")]
pub type Priority20 = crate::Reg<priority20::Priority20Spec>;
#[doc = "Priority register for interrupt source 20"]
pub mod priority20;
#[doc = "PRIORITY21 (rw) register accessor: Priority register for interrupt source 21\n\nYou can [`read`](crate::Reg::read) this register and get [`priority21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority21`]
module"]
#[doc(alias = "PRIORITY21")]
pub type Priority21 = crate::Reg<priority21::Priority21Spec>;
#[doc = "Priority register for interrupt source 21"]
pub mod priority21;
#[doc = "PRIORITY22 (rw) register accessor: Priority register for interrupt source 22\n\nYou can [`read`](crate::Reg::read) this register and get [`priority22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority22`]
module"]
#[doc(alias = "PRIORITY22")]
pub type Priority22 = crate::Reg<priority22::Priority22Spec>;
#[doc = "Priority register for interrupt source 22"]
pub mod priority22;
#[doc = "PRIORITY23 (rw) register accessor: Priority register for interrupt source 23\n\nYou can [`read`](crate::Reg::read) this register and get [`priority23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority23`]
module"]
#[doc(alias = "PRIORITY23")]
pub type Priority23 = crate::Reg<priority23::Priority23Spec>;
#[doc = "Priority register for interrupt source 23"]
pub mod priority23;
#[doc = "PRIORITY24 (rw) register accessor: Priority register for interrupt source 24\n\nYou can [`read`](crate::Reg::read) this register and get [`priority24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority24`]
module"]
#[doc(alias = "PRIORITY24")]
pub type Priority24 = crate::Reg<priority24::Priority24Spec>;
#[doc = "Priority register for interrupt source 24"]
pub mod priority24;
#[doc = "PRIORITY25 (rw) register accessor: Priority register for interrupt source 25\n\nYou can [`read`](crate::Reg::read) this register and get [`priority25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority25`]
module"]
#[doc(alias = "PRIORITY25")]
pub type Priority25 = crate::Reg<priority25::Priority25Spec>;
#[doc = "Priority register for interrupt source 25"]
pub mod priority25;
#[doc = "PRIORITY26 (rw) register accessor: Priority register for interrupt source 26\n\nYou can [`read`](crate::Reg::read) this register and get [`priority26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority26`]
module"]
#[doc(alias = "PRIORITY26")]
pub type Priority26 = crate::Reg<priority26::Priority26Spec>;
#[doc = "Priority register for interrupt source 26"]
pub mod priority26;
#[doc = "PRIORITY27 (rw) register accessor: Priority register for interrupt source 27\n\nYou can [`read`](crate::Reg::read) this register and get [`priority27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority27`]
module"]
#[doc(alias = "PRIORITY27")]
pub type Priority27 = crate::Reg<priority27::Priority27Spec>;
#[doc = "Priority register for interrupt source 27"]
pub mod priority27;
#[doc = "PRIORITY28 (rw) register accessor: Priority register for interrupt source 28\n\nYou can [`read`](crate::Reg::read) this register and get [`priority28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority28`]
module"]
#[doc(alias = "PRIORITY28")]
pub type Priority28 = crate::Reg<priority28::Priority28Spec>;
#[doc = "Priority register for interrupt source 28"]
pub mod priority28;
#[doc = "PRIORITY29 (rw) register accessor: Priority register for interrupt source 29\n\nYou can [`read`](crate::Reg::read) this register and get [`priority29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority29`]
module"]
#[doc(alias = "PRIORITY29")]
pub type Priority29 = crate::Reg<priority29::Priority29Spec>;
#[doc = "Priority register for interrupt source 29"]
pub mod priority29;
#[doc = "PRIORITY30 (rw) register accessor: Priority register for interrupt source 30\n\nYou can [`read`](crate::Reg::read) this register and get [`priority30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority30`]
module"]
#[doc(alias = "PRIORITY30")]
pub type Priority30 = crate::Reg<priority30::Priority30Spec>;
#[doc = "Priority register for interrupt source 30"]
pub mod priority30;
#[doc = "PRIORITY31 (rw) register accessor: Priority register for interrupt source 31\n\nYou can [`read`](crate::Reg::read) this register and get [`priority31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority31`]
module"]
#[doc(alias = "PRIORITY31")]
pub type Priority31 = crate::Reg<priority31::Priority31Spec>;
#[doc = "Priority register for interrupt source 31"]
pub mod priority31;
#[doc = "PRIORITY32 (rw) register accessor: Priority register for interrupt source 32\n\nYou can [`read`](crate::Reg::read) this register and get [`priority32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority32`]
module"]
#[doc(alias = "PRIORITY32")]
pub type Priority32 = crate::Reg<priority32::Priority32Spec>;
#[doc = "Priority register for interrupt source 32"]
pub mod priority32;
#[doc = "PRIORITY33 (rw) register accessor: Priority register for interrupt source 33\n\nYou can [`read`](crate::Reg::read) this register and get [`priority33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority33`]
module"]
#[doc(alias = "PRIORITY33")]
pub type Priority33 = crate::Reg<priority33::Priority33Spec>;
#[doc = "Priority register for interrupt source 33"]
pub mod priority33;
#[doc = "PRIORITY34 (rw) register accessor: Priority register for interrupt source 34\n\nYou can [`read`](crate::Reg::read) this register and get [`priority34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority34`]
module"]
#[doc(alias = "PRIORITY34")]
pub type Priority34 = crate::Reg<priority34::Priority34Spec>;
#[doc = "Priority register for interrupt source 34"]
pub mod priority34;
#[doc = "PRIORITY35 (rw) register accessor: Priority register for interrupt source 35\n\nYou can [`read`](crate::Reg::read) this register and get [`priority35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority35`]
module"]
#[doc(alias = "PRIORITY35")]
pub type Priority35 = crate::Reg<priority35::Priority35Spec>;
#[doc = "Priority register for interrupt source 35"]
pub mod priority35;
#[doc = "PRIORITY36 (rw) register accessor: Priority register for interrupt source 36\n\nYou can [`read`](crate::Reg::read) this register and get [`priority36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority36`]
module"]
#[doc(alias = "PRIORITY36")]
pub type Priority36 = crate::Reg<priority36::Priority36Spec>;
#[doc = "Priority register for interrupt source 36"]
pub mod priority36;
#[doc = "PRIORITY37 (rw) register accessor: Priority register for interrupt source 37\n\nYou can [`read`](crate::Reg::read) this register and get [`priority37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority37`]
module"]
#[doc(alias = "PRIORITY37")]
pub type Priority37 = crate::Reg<priority37::Priority37Spec>;
#[doc = "Priority register for interrupt source 37"]
pub mod priority37;
#[doc = "PRIORITY38 (rw) register accessor: Priority register for interrupt source 38\n\nYou can [`read`](crate::Reg::read) this register and get [`priority38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority38`]
module"]
#[doc(alias = "PRIORITY38")]
pub type Priority38 = crate::Reg<priority38::Priority38Spec>;
#[doc = "Priority register for interrupt source 38"]
pub mod priority38;
#[doc = "PRIORITY39 (rw) register accessor: Priority register for interrupt source 39\n\nYou can [`read`](crate::Reg::read) this register and get [`priority39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority39`]
module"]
#[doc(alias = "PRIORITY39")]
pub type Priority39 = crate::Reg<priority39::Priority39Spec>;
#[doc = "Priority register for interrupt source 39"]
pub mod priority39;
#[doc = "PRIORITY40 (rw) register accessor: Priority register for interrupt source 40\n\nYou can [`read`](crate::Reg::read) this register and get [`priority40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority40`]
module"]
#[doc(alias = "PRIORITY40")]
pub type Priority40 = crate::Reg<priority40::Priority40Spec>;
#[doc = "Priority register for interrupt source 40"]
pub mod priority40;
#[doc = "PRIORITY41 (rw) register accessor: Priority register for interrupt source 41\n\nYou can [`read`](crate::Reg::read) this register and get [`priority41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority41`]
module"]
#[doc(alias = "PRIORITY41")]
pub type Priority41 = crate::Reg<priority41::Priority41Spec>;
#[doc = "Priority register for interrupt source 41"]
pub mod priority41;
#[doc = "PRIORITY42 (rw) register accessor: Priority register for interrupt source 42\n\nYou can [`read`](crate::Reg::read) this register and get [`priority42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority42`]
module"]
#[doc(alias = "PRIORITY42")]
pub type Priority42 = crate::Reg<priority42::Priority42Spec>;
#[doc = "Priority register for interrupt source 42"]
pub mod priority42;
#[doc = "PRIORITY43 (rw) register accessor: Priority register for interrupt source 43\n\nYou can [`read`](crate::Reg::read) this register and get [`priority43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority43`]
module"]
#[doc(alias = "PRIORITY43")]
pub type Priority43 = crate::Reg<priority43::Priority43Spec>;
#[doc = "Priority register for interrupt source 43"]
pub mod priority43;
#[doc = "PRIORITY44 (rw) register accessor: Priority register for interrupt source 44\n\nYou can [`read`](crate::Reg::read) this register and get [`priority44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority44`]
module"]
#[doc(alias = "PRIORITY44")]
pub type Priority44 = crate::Reg<priority44::Priority44Spec>;
#[doc = "Priority register for interrupt source 44"]
pub mod priority44;
#[doc = "PRIORITY45 (rw) register accessor: Priority register for interrupt source 45\n\nYou can [`read`](crate::Reg::read) this register and get [`priority45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority45`]
module"]
#[doc(alias = "PRIORITY45")]
pub type Priority45 = crate::Reg<priority45::Priority45Spec>;
#[doc = "Priority register for interrupt source 45"]
pub mod priority45;
#[doc = "PRIORITY46 (rw) register accessor: Priority register for interrupt source 46\n\nYou can [`read`](crate::Reg::read) this register and get [`priority46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority46`]
module"]
#[doc(alias = "PRIORITY46")]
pub type Priority46 = crate::Reg<priority46::Priority46Spec>;
#[doc = "Priority register for interrupt source 46"]
pub mod priority46;
#[doc = "PRIORITY47 (rw) register accessor: Priority register for interrupt source 47\n\nYou can [`read`](crate::Reg::read) this register and get [`priority47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority47`]
module"]
#[doc(alias = "PRIORITY47")]
pub type Priority47 = crate::Reg<priority47::Priority47Spec>;
#[doc = "Priority register for interrupt source 47"]
pub mod priority47;
#[doc = "PRIORITY48 (rw) register accessor: Priority register for interrupt source 48\n\nYou can [`read`](crate::Reg::read) this register and get [`priority48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority48`]
module"]
#[doc(alias = "PRIORITY48")]
pub type Priority48 = crate::Reg<priority48::Priority48Spec>;
#[doc = "Priority register for interrupt source 48"]
pub mod priority48;
#[doc = "PRIORITY49 (rw) register accessor: Priority register for interrupt source 49\n\nYou can [`read`](crate::Reg::read) this register and get [`priority49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority49`]
module"]
#[doc(alias = "PRIORITY49")]
pub type Priority49 = crate::Reg<priority49::Priority49Spec>;
#[doc = "Priority register for interrupt source 49"]
pub mod priority49;
#[doc = "PRIORITY50 (rw) register accessor: Priority register for interrupt source 50\n\nYou can [`read`](crate::Reg::read) this register and get [`priority50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority50`]
module"]
#[doc(alias = "PRIORITY50")]
pub type Priority50 = crate::Reg<priority50::Priority50Spec>;
#[doc = "Priority register for interrupt source 50"]
pub mod priority50;
#[doc = "PRIORITY51 (rw) register accessor: Priority register for interrupt source 51\n\nYou can [`read`](crate::Reg::read) this register and get [`priority51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority51`]
module"]
#[doc(alias = "PRIORITY51")]
pub type Priority51 = crate::Reg<priority51::Priority51Spec>;
#[doc = "Priority register for interrupt source 51"]
pub mod priority51;
#[doc = "PRIORITY52 (rw) register accessor: Priority register for interrupt source 52\n\nYou can [`read`](crate::Reg::read) this register and get [`priority52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority52`]
module"]
#[doc(alias = "PRIORITY52")]
pub type Priority52 = crate::Reg<priority52::Priority52Spec>;
#[doc = "Priority register for interrupt source 52"]
pub mod priority52;
#[doc = "PRIORITY53 (rw) register accessor: Priority register for interrupt source 53\n\nYou can [`read`](crate::Reg::read) this register and get [`priority53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority53`]
module"]
#[doc(alias = "PRIORITY53")]
pub type Priority53 = crate::Reg<priority53::Priority53Spec>;
#[doc = "Priority register for interrupt source 53"]
pub mod priority53;
#[doc = "PRIORITY54 (rw) register accessor: Priority register for interrupt source 54\n\nYou can [`read`](crate::Reg::read) this register and get [`priority54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority54`]
module"]
#[doc(alias = "PRIORITY54")]
pub type Priority54 = crate::Reg<priority54::Priority54Spec>;
#[doc = "Priority register for interrupt source 54"]
pub mod priority54;
#[doc = "PRIORITY55 (rw) register accessor: Priority register for interrupt source 55\n\nYou can [`read`](crate::Reg::read) this register and get [`priority55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority55`]
module"]
#[doc(alias = "PRIORITY55")]
pub type Priority55 = crate::Reg<priority55::Priority55Spec>;
#[doc = "Priority register for interrupt source 55"]
pub mod priority55;
#[doc = "PRIORITY56 (rw) register accessor: Priority register for interrupt source 56\n\nYou can [`read`](crate::Reg::read) this register and get [`priority56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority56`]
module"]
#[doc(alias = "PRIORITY56")]
pub type Priority56 = crate::Reg<priority56::Priority56Spec>;
#[doc = "Priority register for interrupt source 56"]
pub mod priority56;
#[doc = "PRIORITY57 (rw) register accessor: Priority register for interrupt source 57\n\nYou can [`read`](crate::Reg::read) this register and get [`priority57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority57`]
module"]
#[doc(alias = "PRIORITY57")]
pub type Priority57 = crate::Reg<priority57::Priority57Spec>;
#[doc = "Priority register for interrupt source 57"]
pub mod priority57;
#[doc = "PRIORITY58 (rw) register accessor: Priority register for interrupt source 58\n\nYou can [`read`](crate::Reg::read) this register and get [`priority58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority58`]
module"]
#[doc(alias = "PRIORITY58")]
pub type Priority58 = crate::Reg<priority58::Priority58Spec>;
#[doc = "Priority register for interrupt source 58"]
pub mod priority58;
#[doc = "PRIORITY59 (rw) register accessor: Priority register for interrupt source 59\n\nYou can [`read`](crate::Reg::read) this register and get [`priority59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority59`]
module"]
#[doc(alias = "PRIORITY59")]
pub type Priority59 = crate::Reg<priority59::Priority59Spec>;
#[doc = "Priority register for interrupt source 59"]
pub mod priority59;
#[doc = "PRIORITY60 (rw) register accessor: Priority register for interrupt source 60\n\nYou can [`read`](crate::Reg::read) this register and get [`priority60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority60`]
module"]
#[doc(alias = "PRIORITY60")]
pub type Priority60 = crate::Reg<priority60::Priority60Spec>;
#[doc = "Priority register for interrupt source 60"]
pub mod priority60;
#[doc = "PRIORITY61 (rw) register accessor: Priority register for interrupt source 61\n\nYou can [`read`](crate::Reg::read) this register and get [`priority61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority61`]
module"]
#[doc(alias = "PRIORITY61")]
pub type Priority61 = crate::Reg<priority61::Priority61Spec>;
#[doc = "Priority register for interrupt source 61"]
pub mod priority61;
#[doc = "PRIORITY62 (rw) register accessor: Priority register for interrupt source 62\n\nYou can [`read`](crate::Reg::read) this register and get [`priority62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority62`]
module"]
#[doc(alias = "PRIORITY62")]
pub type Priority62 = crate::Reg<priority62::Priority62Spec>;
#[doc = "Priority register for interrupt source 62"]
pub mod priority62;
#[doc = "PRIORITY63 (rw) register accessor: Priority register for interrupt source 63\n\nYou can [`read`](crate::Reg::read) this register and get [`priority63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority63`]
module"]
#[doc(alias = "PRIORITY63")]
pub type Priority63 = crate::Reg<priority63::Priority63Spec>;
#[doc = "Priority register for interrupt source 63"]
pub mod priority63;
#[doc = "PENDING_0_32 (r) register accessor: Interrupt pending bits of sources 0-32\n\nYou can [`read`](crate::Reg::read) this register and get [`pending_0_32::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending_0_32`]
module"]
#[doc(alias = "PENDING_0_32")]
pub type Pending0_32 = crate::Reg<pending_0_32::Pending0_32Spec>;
#[doc = "Interrupt pending bits of sources 0-32"]
pub mod pending_0_32;
#[doc = "PENDING_33_63 (r) register accessor: Interrupt pending bits of sources 33-63\n\nYou can [`read`](crate::Reg::read) this register and get [`pending_33_63::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending_33_63`]
module"]
#[doc(alias = "PENDING_33_63")]
pub type Pending33_63 = crate::Reg<pending_33_63::Pending33_63Spec>;
#[doc = "Interrupt pending bits of sources 33-63"]
pub mod pending_33_63;
#[doc = "INTR_EN_0_32 (rw) register accessor: Interrupt enable bits of sources 0-32\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_0_32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_0_32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en_0_32`]
module"]
#[doc(alias = "INTR_EN_0_32")]
pub type IntrEn0_32 = crate::Reg<intr_en_0_32::IntrEn0_32Spec>;
#[doc = "Interrupt enable bits of sources 0-32"]
pub mod intr_en_0_32;
#[doc = "INTR_EN_33_63 (rw) register accessor: Interrupt enable bits of sources 33-63\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_33_63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_33_63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en_33_63`]
module"]
#[doc(alias = "INTR_EN_33_63")]
pub type IntrEn33_63 = crate::Reg<intr_en_33_63::IntrEn33_63Spec>;
#[doc = "Interrupt enable bits of sources 33-63"]
pub mod intr_en_33_63;
#[doc = "PRIORITY_THRES (rw) register accessor: Priority threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`priority_thres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority_thres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority_thres`]
module"]
#[doc(alias = "PRIORITY_THRES")]
pub type PriorityThres = crate::Reg<priority_thres::PriorityThresSpec>;
#[doc = "Priority threshold register"]
pub mod priority_thres;
#[doc = "INTR_COMPLETE (rw) register accessor: Interrupt claim/complete register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_complete::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_complete::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_complete`]
module"]
#[doc(alias = "INTR_COMPLETE")]
pub type IntrComplete = crate::Reg<intr_complete::IntrCompleteSpec>;
#[doc = "Interrupt claim/complete register"]
pub mod intr_complete;
