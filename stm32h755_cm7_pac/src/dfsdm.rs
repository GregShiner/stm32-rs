#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch0cfgr1: Ch0cfgr1,
    ch0cfgr2: Ch0cfgr2,
    ch0awscdr: Ch0awscdr,
    ch0wdatr: Ch0wdatr,
    ch0datinr: Ch0datinr,
    ch0dlyr: Ch0dlyr,
    _reserved6: [u8; 0x08],
    ch1cfgr1: Ch1cfgr1,
    ch1cfgr2: Ch1cfgr2,
    ch1awscdr: Ch1awscdr,
    ch1wdatr: Ch1wdatr,
    ch1datinr: Ch1datinr,
    ch1dlyr: Ch1dlyr,
    _reserved12: [u8; 0x08],
    ch2cfgr1: Ch2cfgr1,
    ch2cfgr2: Ch2cfgr2,
    ch2awscdr: Ch2awscdr,
    ch2wdatr: Ch2wdatr,
    ch2datinr: Ch2datinr,
    ch2dlyr: Ch2dlyr,
    _reserved18: [u8; 0x08],
    ch3cfgr1: Ch3cfgr1,
    ch3cfgr2: Ch3cfgr2,
    ch3awscdr: Ch3awscdr,
    ch3wdatr: Ch3wdatr,
    ch3datinr: Ch3datinr,
    ch3dlyr: Ch3dlyr,
    _reserved24: [u8; 0x08],
    ch4cfgr1: Ch4cfgr1,
    ch4cfgr2: Ch4cfgr2,
    ch4awscdr: Ch4awscdr,
    ch4wdatr: Ch4wdatr,
    ch4datinr: Ch4datinr,
    ch4dlyr: Ch4dlyr,
    _reserved30: [u8; 0x08],
    ch5cfgr1: Ch5cfgr1,
    ch5cfgr2: Ch5cfgr2,
    ch5awscdr: Ch5awscdr,
    ch5wdatr: Ch5wdatr,
    ch5datinr: Ch5datinr,
    ch5dlyr: Ch5dlyr,
    _reserved36: [u8; 0x08],
    ch6cfgr1: Ch6cfgr1,
    ch6cfgr2: Ch6cfgr2,
    ch6awscdr: Ch6awscdr,
    ch6wdatr: Ch6wdatr,
    ch6datinr: Ch6datinr,
    ch6dlyr: Ch6dlyr,
    _reserved42: [u8; 0x08],
    ch7cfgr1: Ch7cfgr1,
    ch7cfgr2: Ch7cfgr2,
    ch7awscdr: Ch7awscdr,
    ch7wdatr: Ch7wdatr,
    ch7datinr: Ch7datinr,
    ch7dlyr: Ch7dlyr,
    _reserved48: [u8; 0x08],
    dfsdm_flt0cr1: DfsdmFlt0cr1,
    dfsdm_flt0cr2: DfsdmFlt0cr2,
    dfsdm_flt0isr: DfsdmFlt0isr,
    dfsdm_flt0icr: DfsdmFlt0icr,
    dfsdm_flt0jchgr: DfsdmFlt0jchgr,
    dfsdm_flt0fcr: DfsdmFlt0fcr,
    dfsdm_flt0jdatar: DfsdmFlt0jdatar,
    dfsdm_flt0rdatar: DfsdmFlt0rdatar,
    dfsdm_flt0awhtr: DfsdmFlt0awhtr,
    dfsdm_flt0awltr: DfsdmFlt0awltr,
    dfsdm_flt0awsr: DfsdmFlt0awsr,
    dfsdm_flt0awcfr: DfsdmFlt0awcfr,
    dfsdm_flt0exmax: DfsdmFlt0exmax,
    dfsdm_flt0exmin: DfsdmFlt0exmin,
    dfsdm_flt0cnvtimr: DfsdmFlt0cnvtimr,
    _reserved63: [u8; 0x44],
    dfsdm_flt1cr1: DfsdmFlt1cr1,
    dfsdm_flt1cr2: DfsdmFlt1cr2,
    dfsdm_flt1isr: DfsdmFlt1isr,
    dfsdm_flt1icr: DfsdmFlt1icr,
    dfsdm_flt1chgr: DfsdmFlt1chgr,
    dfsdm_flt1fcr: DfsdmFlt1fcr,
    dfsdm_flt1jdatar: DfsdmFlt1jdatar,
    dfsdm_flt1rdatar: DfsdmFlt1rdatar,
    dfsdm_flt1awhtr: DfsdmFlt1awhtr,
    dfsdm_flt1awltr: DfsdmFlt1awltr,
    dfsdm_flt1awsr: DfsdmFlt1awsr,
    dfsdm_flt1awcfr: DfsdmFlt1awcfr,
    dfsdm_flt1exmax: DfsdmFlt1exmax,
    dfsdm_flt1exmin: DfsdmFlt1exmin,
    dfsdm_flt1cnvtimr: DfsdmFlt1cnvtimr,
    _reserved78: [u8; 0x44],
    dfsdm_flt2cr1: DfsdmFlt2cr1,
    dfsdm_flt2cr2: DfsdmFlt2cr2,
    dfsdm_flt2isr: DfsdmFlt2isr,
    dfsdm_flt2icr: DfsdmFlt2icr,
    dfsdm_flt2jchgr: DfsdmFlt2jchgr,
    dfsdm_flt2fcr: DfsdmFlt2fcr,
    dfsdm_flt2jdatar: DfsdmFlt2jdatar,
    dfsdm_flt2rdatar: DfsdmFlt2rdatar,
    dfsdm_flt2awhtr: DfsdmFlt2awhtr,
    dfsdm_flt2awltr: DfsdmFlt2awltr,
    dfsdm_flt2awsr: DfsdmFlt2awsr,
    dfsdm_flt2awcfr: DfsdmFlt2awcfr,
    dfsdm_flt2exmax: DfsdmFlt2exmax,
    dfsdm_flt2exmin: DfsdmFlt2exmin,
    dfsdm_flt2cnvtimr: DfsdmFlt2cnvtimr,
    _reserved93: [u8; 0x44],
    dfsdm_flt3cr1: DfsdmFlt3cr1,
    dfsdm_flt3cr2: DfsdmFlt3cr2,
    dfsdm_flt3isr: DfsdmFlt3isr,
    dfsdm_flt3icr: DfsdmFlt3icr,
    dfsdm_flt3jchgr: DfsdmFlt3jchgr,
    dfsdm_flt3fcr: DfsdmFlt3fcr,
    dfsdm_flt3jdatar: DfsdmFlt3jdatar,
    dfsdm_flt3rdatar: DfsdmFlt3rdatar,
    dfsdm_flt3awhtr: DfsdmFlt3awhtr,
    dfsdm_flt3awltr: DfsdmFlt3awltr,
    dfsdm_flt3awsr: DfsdmFlt3awsr,
    dfsdm_flt3awcfr: DfsdmFlt3awcfr,
    dfsdm_flt3exmax: DfsdmFlt3exmax,
    dfsdm_flt3exmin: DfsdmFlt3exmin,
    dfsdm_flt3cnvtimr: DfsdmFlt3cnvtimr,
}
impl RegisterBlock {
    #[doc = "0x00 - channel configuration y register"]
    #[inline(always)]
    pub const fn ch0cfgr1(&self) -> &Ch0cfgr1 {
        &self.ch0cfgr1
    }
    #[doc = "0x04 - channel configuration y register"]
    #[inline(always)]
    pub const fn ch0cfgr2(&self) -> &Ch0cfgr2 {
        &self.ch0cfgr2
    }
    #[doc = "0x08 - analog watchdog and short-circuit detector register"]
    #[inline(always)]
    pub const fn ch0awscdr(&self) -> &Ch0awscdr {
        &self.ch0awscdr
    }
    #[doc = "0x0c - channel watchdog filter data register"]
    #[inline(always)]
    pub const fn ch0wdatr(&self) -> &Ch0wdatr {
        &self.ch0wdatr
    }
    #[doc = "0x10 - channel data input register"]
    #[inline(always)]
    pub const fn ch0datinr(&self) -> &Ch0datinr {
        &self.ch0datinr
    }
    #[doc = "0x14 - channel y delay register"]
    #[inline(always)]
    pub const fn ch0dlyr(&self) -> &Ch0dlyr {
        &self.ch0dlyr
    }
    #[doc = "0x20 - CH1CFGR1"]
    #[inline(always)]
    pub const fn ch1cfgr1(&self) -> &Ch1cfgr1 {
        &self.ch1cfgr1
    }
    #[doc = "0x24 - CH1CFGR2"]
    #[inline(always)]
    pub const fn ch1cfgr2(&self) -> &Ch1cfgr2 {
        &self.ch1cfgr2
    }
    #[doc = "0x28 - CH1AWSCDR"]
    #[inline(always)]
    pub const fn ch1awscdr(&self) -> &Ch1awscdr {
        &self.ch1awscdr
    }
    #[doc = "0x2c - CH1WDATR"]
    #[inline(always)]
    pub const fn ch1wdatr(&self) -> &Ch1wdatr {
        &self.ch1wdatr
    }
    #[doc = "0x30 - CH1DATINR"]
    #[inline(always)]
    pub const fn ch1datinr(&self) -> &Ch1datinr {
        &self.ch1datinr
    }
    #[doc = "0x34 - channel y delay register"]
    #[inline(always)]
    pub const fn ch1dlyr(&self) -> &Ch1dlyr {
        &self.ch1dlyr
    }
    #[doc = "0x40 - CH2CFGR1"]
    #[inline(always)]
    pub const fn ch2cfgr1(&self) -> &Ch2cfgr1 {
        &self.ch2cfgr1
    }
    #[doc = "0x44 - CH2CFGR2"]
    #[inline(always)]
    pub const fn ch2cfgr2(&self) -> &Ch2cfgr2 {
        &self.ch2cfgr2
    }
    #[doc = "0x48 - CH2AWSCDR"]
    #[inline(always)]
    pub const fn ch2awscdr(&self) -> &Ch2awscdr {
        &self.ch2awscdr
    }
    #[doc = "0x4c - CH2WDATR"]
    #[inline(always)]
    pub const fn ch2wdatr(&self) -> &Ch2wdatr {
        &self.ch2wdatr
    }
    #[doc = "0x50 - CH2DATINR"]
    #[inline(always)]
    pub const fn ch2datinr(&self) -> &Ch2datinr {
        &self.ch2datinr
    }
    #[doc = "0x54 - channel y delay register"]
    #[inline(always)]
    pub const fn ch2dlyr(&self) -> &Ch2dlyr {
        &self.ch2dlyr
    }
    #[doc = "0x60 - CH3CFGR1"]
    #[inline(always)]
    pub const fn ch3cfgr1(&self) -> &Ch3cfgr1 {
        &self.ch3cfgr1
    }
    #[doc = "0x64 - CH3CFGR2"]
    #[inline(always)]
    pub const fn ch3cfgr2(&self) -> &Ch3cfgr2 {
        &self.ch3cfgr2
    }
    #[doc = "0x68 - CH3AWSCDR"]
    #[inline(always)]
    pub const fn ch3awscdr(&self) -> &Ch3awscdr {
        &self.ch3awscdr
    }
    #[doc = "0x6c - CH3WDATR"]
    #[inline(always)]
    pub const fn ch3wdatr(&self) -> &Ch3wdatr {
        &self.ch3wdatr
    }
    #[doc = "0x70 - CH3DATINR"]
    #[inline(always)]
    pub const fn ch3datinr(&self) -> &Ch3datinr {
        &self.ch3datinr
    }
    #[doc = "0x74 - channel y delay register"]
    #[inline(always)]
    pub const fn ch3dlyr(&self) -> &Ch3dlyr {
        &self.ch3dlyr
    }
    #[doc = "0x80 - CH4CFGR1"]
    #[inline(always)]
    pub const fn ch4cfgr1(&self) -> &Ch4cfgr1 {
        &self.ch4cfgr1
    }
    #[doc = "0x84 - CH4CFGR2"]
    #[inline(always)]
    pub const fn ch4cfgr2(&self) -> &Ch4cfgr2 {
        &self.ch4cfgr2
    }
    #[doc = "0x88 - CH4AWSCDR"]
    #[inline(always)]
    pub const fn ch4awscdr(&self) -> &Ch4awscdr {
        &self.ch4awscdr
    }
    #[doc = "0x8c - CH4WDATR"]
    #[inline(always)]
    pub const fn ch4wdatr(&self) -> &Ch4wdatr {
        &self.ch4wdatr
    }
    #[doc = "0x90 - CH4DATINR"]
    #[inline(always)]
    pub const fn ch4datinr(&self) -> &Ch4datinr {
        &self.ch4datinr
    }
    #[doc = "0x94 - channel y delay register"]
    #[inline(always)]
    pub const fn ch4dlyr(&self) -> &Ch4dlyr {
        &self.ch4dlyr
    }
    #[doc = "0xa0 - CH5CFGR1"]
    #[inline(always)]
    pub const fn ch5cfgr1(&self) -> &Ch5cfgr1 {
        &self.ch5cfgr1
    }
    #[doc = "0xa4 - CH5CFGR2"]
    #[inline(always)]
    pub const fn ch5cfgr2(&self) -> &Ch5cfgr2 {
        &self.ch5cfgr2
    }
    #[doc = "0xa8 - CH5AWSCDR"]
    #[inline(always)]
    pub const fn ch5awscdr(&self) -> &Ch5awscdr {
        &self.ch5awscdr
    }
    #[doc = "0xac - CH5WDATR"]
    #[inline(always)]
    pub const fn ch5wdatr(&self) -> &Ch5wdatr {
        &self.ch5wdatr
    }
    #[doc = "0xb0 - CH5DATINR"]
    #[inline(always)]
    pub const fn ch5datinr(&self) -> &Ch5datinr {
        &self.ch5datinr
    }
    #[doc = "0xb4 - channel y delay register"]
    #[inline(always)]
    pub const fn ch5dlyr(&self) -> &Ch5dlyr {
        &self.ch5dlyr
    }
    #[doc = "0xc0 - CH6CFGR1"]
    #[inline(always)]
    pub const fn ch6cfgr1(&self) -> &Ch6cfgr1 {
        &self.ch6cfgr1
    }
    #[doc = "0xc4 - CH6CFGR2"]
    #[inline(always)]
    pub const fn ch6cfgr2(&self) -> &Ch6cfgr2 {
        &self.ch6cfgr2
    }
    #[doc = "0xc8 - CH6AWSCDR"]
    #[inline(always)]
    pub const fn ch6awscdr(&self) -> &Ch6awscdr {
        &self.ch6awscdr
    }
    #[doc = "0xcc - CH6WDATR"]
    #[inline(always)]
    pub const fn ch6wdatr(&self) -> &Ch6wdatr {
        &self.ch6wdatr
    }
    #[doc = "0xd0 - CH6DATINR"]
    #[inline(always)]
    pub const fn ch6datinr(&self) -> &Ch6datinr {
        &self.ch6datinr
    }
    #[doc = "0xd4 - channel y delay register"]
    #[inline(always)]
    pub const fn ch6dlyr(&self) -> &Ch6dlyr {
        &self.ch6dlyr
    }
    #[doc = "0xe0 - CH7CFGR1"]
    #[inline(always)]
    pub const fn ch7cfgr1(&self) -> &Ch7cfgr1 {
        &self.ch7cfgr1
    }
    #[doc = "0xe4 - CH7CFGR2"]
    #[inline(always)]
    pub const fn ch7cfgr2(&self) -> &Ch7cfgr2 {
        &self.ch7cfgr2
    }
    #[doc = "0xe8 - CH7AWSCDR"]
    #[inline(always)]
    pub const fn ch7awscdr(&self) -> &Ch7awscdr {
        &self.ch7awscdr
    }
    #[doc = "0xec - CH7WDATR"]
    #[inline(always)]
    pub const fn ch7wdatr(&self) -> &Ch7wdatr {
        &self.ch7wdatr
    }
    #[doc = "0xf0 - CH7DATINR"]
    #[inline(always)]
    pub const fn ch7datinr(&self) -> &Ch7datinr {
        &self.ch7datinr
    }
    #[doc = "0xf4 - channel y delay register"]
    #[inline(always)]
    pub const fn ch7dlyr(&self) -> &Ch7dlyr {
        &self.ch7dlyr
    }
    #[doc = "0x100 - control register 1"]
    #[inline(always)]
    pub const fn dfsdm_flt0cr1(&self) -> &DfsdmFlt0cr1 {
        &self.dfsdm_flt0cr1
    }
    #[doc = "0x104 - control register 2"]
    #[inline(always)]
    pub const fn dfsdm_flt0cr2(&self) -> &DfsdmFlt0cr2 {
        &self.dfsdm_flt0cr2
    }
    #[doc = "0x108 - interrupt and status register"]
    #[inline(always)]
    pub const fn dfsdm_flt0isr(&self) -> &DfsdmFlt0isr {
        &self.dfsdm_flt0isr
    }
    #[doc = "0x10c - interrupt flag clear register"]
    #[inline(always)]
    pub const fn dfsdm_flt0icr(&self) -> &DfsdmFlt0icr {
        &self.dfsdm_flt0icr
    }
    #[doc = "0x110 - injected channel group selection register"]
    #[inline(always)]
    pub const fn dfsdm_flt0jchgr(&self) -> &DfsdmFlt0jchgr {
        &self.dfsdm_flt0jchgr
    }
    #[doc = "0x114 - filter control register"]
    #[inline(always)]
    pub const fn dfsdm_flt0fcr(&self) -> &DfsdmFlt0fcr {
        &self.dfsdm_flt0fcr
    }
    #[doc = "0x118 - data register for injected group"]
    #[inline(always)]
    pub const fn dfsdm_flt0jdatar(&self) -> &DfsdmFlt0jdatar {
        &self.dfsdm_flt0jdatar
    }
    #[doc = "0x11c - data register for the regular channel"]
    #[inline(always)]
    pub const fn dfsdm_flt0rdatar(&self) -> &DfsdmFlt0rdatar {
        &self.dfsdm_flt0rdatar
    }
    #[doc = "0x120 - analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt0awhtr(&self) -> &DfsdmFlt0awhtr {
        &self.dfsdm_flt0awhtr
    }
    #[doc = "0x124 - analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt0awltr(&self) -> &DfsdmFlt0awltr {
        &self.dfsdm_flt0awltr
    }
    #[doc = "0x128 - analog watchdog status register"]
    #[inline(always)]
    pub const fn dfsdm_flt0awsr(&self) -> &DfsdmFlt0awsr {
        &self.dfsdm_flt0awsr
    }
    #[doc = "0x12c - analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn dfsdm_flt0awcfr(&self) -> &DfsdmFlt0awcfr {
        &self.dfsdm_flt0awcfr
    }
    #[doc = "0x130 - Extremes detector maximum register"]
    #[inline(always)]
    pub const fn dfsdm_flt0exmax(&self) -> &DfsdmFlt0exmax {
        &self.dfsdm_flt0exmax
    }
    #[doc = "0x134 - Extremes detector minimum register"]
    #[inline(always)]
    pub const fn dfsdm_flt0exmin(&self) -> &DfsdmFlt0exmin {
        &self.dfsdm_flt0exmin
    }
    #[doc = "0x138 - conversion timer register"]
    #[inline(always)]
    pub const fn dfsdm_flt0cnvtimr(&self) -> &DfsdmFlt0cnvtimr {
        &self.dfsdm_flt0cnvtimr
    }
    #[doc = "0x180 - control register 1"]
    #[inline(always)]
    pub const fn dfsdm_flt1cr1(&self) -> &DfsdmFlt1cr1 {
        &self.dfsdm_flt1cr1
    }
    #[doc = "0x184 - control register 2"]
    #[inline(always)]
    pub const fn dfsdm_flt1cr2(&self) -> &DfsdmFlt1cr2 {
        &self.dfsdm_flt1cr2
    }
    #[doc = "0x188 - interrupt and status register"]
    #[inline(always)]
    pub const fn dfsdm_flt1isr(&self) -> &DfsdmFlt1isr {
        &self.dfsdm_flt1isr
    }
    #[doc = "0x18c - interrupt flag clear register"]
    #[inline(always)]
    pub const fn dfsdm_flt1icr(&self) -> &DfsdmFlt1icr {
        &self.dfsdm_flt1icr
    }
    #[doc = "0x190 - injected channel group selection register"]
    #[inline(always)]
    pub const fn dfsdm_flt1chgr(&self) -> &DfsdmFlt1chgr {
        &self.dfsdm_flt1chgr
    }
    #[doc = "0x194 - filter control register"]
    #[inline(always)]
    pub const fn dfsdm_flt1fcr(&self) -> &DfsdmFlt1fcr {
        &self.dfsdm_flt1fcr
    }
    #[doc = "0x198 - data register for injected group"]
    #[inline(always)]
    pub const fn dfsdm_flt1jdatar(&self) -> &DfsdmFlt1jdatar {
        &self.dfsdm_flt1jdatar
    }
    #[doc = "0x19c - data register for the regular channel"]
    #[inline(always)]
    pub const fn dfsdm_flt1rdatar(&self) -> &DfsdmFlt1rdatar {
        &self.dfsdm_flt1rdatar
    }
    #[doc = "0x1a0 - analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt1awhtr(&self) -> &DfsdmFlt1awhtr {
        &self.dfsdm_flt1awhtr
    }
    #[doc = "0x1a4 - analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt1awltr(&self) -> &DfsdmFlt1awltr {
        &self.dfsdm_flt1awltr
    }
    #[doc = "0x1a8 - analog watchdog status register"]
    #[inline(always)]
    pub const fn dfsdm_flt1awsr(&self) -> &DfsdmFlt1awsr {
        &self.dfsdm_flt1awsr
    }
    #[doc = "0x1ac - analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn dfsdm_flt1awcfr(&self) -> &DfsdmFlt1awcfr {
        &self.dfsdm_flt1awcfr
    }
    #[doc = "0x1b0 - Extremes detector maximum register"]
    #[inline(always)]
    pub const fn dfsdm_flt1exmax(&self) -> &DfsdmFlt1exmax {
        &self.dfsdm_flt1exmax
    }
    #[doc = "0x1b4 - Extremes detector minimum register"]
    #[inline(always)]
    pub const fn dfsdm_flt1exmin(&self) -> &DfsdmFlt1exmin {
        &self.dfsdm_flt1exmin
    }
    #[doc = "0x1b8 - conversion timer register"]
    #[inline(always)]
    pub const fn dfsdm_flt1cnvtimr(&self) -> &DfsdmFlt1cnvtimr {
        &self.dfsdm_flt1cnvtimr
    }
    #[doc = "0x200 - control register 1"]
    #[inline(always)]
    pub const fn dfsdm_flt2cr1(&self) -> &DfsdmFlt2cr1 {
        &self.dfsdm_flt2cr1
    }
    #[doc = "0x204 - control register 2"]
    #[inline(always)]
    pub const fn dfsdm_flt2cr2(&self) -> &DfsdmFlt2cr2 {
        &self.dfsdm_flt2cr2
    }
    #[doc = "0x208 - interrupt and status register"]
    #[inline(always)]
    pub const fn dfsdm_flt2isr(&self) -> &DfsdmFlt2isr {
        &self.dfsdm_flt2isr
    }
    #[doc = "0x20c - interrupt flag clear register"]
    #[inline(always)]
    pub const fn dfsdm_flt2icr(&self) -> &DfsdmFlt2icr {
        &self.dfsdm_flt2icr
    }
    #[doc = "0x210 - injected channel group selection register"]
    #[inline(always)]
    pub const fn dfsdm_flt2jchgr(&self) -> &DfsdmFlt2jchgr {
        &self.dfsdm_flt2jchgr
    }
    #[doc = "0x214 - filter control register"]
    #[inline(always)]
    pub const fn dfsdm_flt2fcr(&self) -> &DfsdmFlt2fcr {
        &self.dfsdm_flt2fcr
    }
    #[doc = "0x218 - data register for injected group"]
    #[inline(always)]
    pub const fn dfsdm_flt2jdatar(&self) -> &DfsdmFlt2jdatar {
        &self.dfsdm_flt2jdatar
    }
    #[doc = "0x21c - data register for the regular channel"]
    #[inline(always)]
    pub const fn dfsdm_flt2rdatar(&self) -> &DfsdmFlt2rdatar {
        &self.dfsdm_flt2rdatar
    }
    #[doc = "0x220 - analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt2awhtr(&self) -> &DfsdmFlt2awhtr {
        &self.dfsdm_flt2awhtr
    }
    #[doc = "0x224 - analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt2awltr(&self) -> &DfsdmFlt2awltr {
        &self.dfsdm_flt2awltr
    }
    #[doc = "0x228 - analog watchdog status register"]
    #[inline(always)]
    pub const fn dfsdm_flt2awsr(&self) -> &DfsdmFlt2awsr {
        &self.dfsdm_flt2awsr
    }
    #[doc = "0x22c - analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn dfsdm_flt2awcfr(&self) -> &DfsdmFlt2awcfr {
        &self.dfsdm_flt2awcfr
    }
    #[doc = "0x230 - Extremes detector maximum register"]
    #[inline(always)]
    pub const fn dfsdm_flt2exmax(&self) -> &DfsdmFlt2exmax {
        &self.dfsdm_flt2exmax
    }
    #[doc = "0x234 - Extremes detector minimum register"]
    #[inline(always)]
    pub const fn dfsdm_flt2exmin(&self) -> &DfsdmFlt2exmin {
        &self.dfsdm_flt2exmin
    }
    #[doc = "0x238 - conversion timer register"]
    #[inline(always)]
    pub const fn dfsdm_flt2cnvtimr(&self) -> &DfsdmFlt2cnvtimr {
        &self.dfsdm_flt2cnvtimr
    }
    #[doc = "0x280 - control register 1"]
    #[inline(always)]
    pub const fn dfsdm_flt3cr1(&self) -> &DfsdmFlt3cr1 {
        &self.dfsdm_flt3cr1
    }
    #[doc = "0x284 - control register 2"]
    #[inline(always)]
    pub const fn dfsdm_flt3cr2(&self) -> &DfsdmFlt3cr2 {
        &self.dfsdm_flt3cr2
    }
    #[doc = "0x288 - interrupt and status register"]
    #[inline(always)]
    pub const fn dfsdm_flt3isr(&self) -> &DfsdmFlt3isr {
        &self.dfsdm_flt3isr
    }
    #[doc = "0x28c - interrupt flag clear register"]
    #[inline(always)]
    pub const fn dfsdm_flt3icr(&self) -> &DfsdmFlt3icr {
        &self.dfsdm_flt3icr
    }
    #[doc = "0x290 - injected channel group selection register"]
    #[inline(always)]
    pub const fn dfsdm_flt3jchgr(&self) -> &DfsdmFlt3jchgr {
        &self.dfsdm_flt3jchgr
    }
    #[doc = "0x294 - filter control register"]
    #[inline(always)]
    pub const fn dfsdm_flt3fcr(&self) -> &DfsdmFlt3fcr {
        &self.dfsdm_flt3fcr
    }
    #[doc = "0x298 - data register for injected group"]
    #[inline(always)]
    pub const fn dfsdm_flt3jdatar(&self) -> &DfsdmFlt3jdatar {
        &self.dfsdm_flt3jdatar
    }
    #[doc = "0x29c - data register for the regular channel"]
    #[inline(always)]
    pub const fn dfsdm_flt3rdatar(&self) -> &DfsdmFlt3rdatar {
        &self.dfsdm_flt3rdatar
    }
    #[doc = "0x2a0 - analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt3awhtr(&self) -> &DfsdmFlt3awhtr {
        &self.dfsdm_flt3awhtr
    }
    #[doc = "0x2a4 - analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt3awltr(&self) -> &DfsdmFlt3awltr {
        &self.dfsdm_flt3awltr
    }
    #[doc = "0x2a8 - analog watchdog status register"]
    #[inline(always)]
    pub const fn dfsdm_flt3awsr(&self) -> &DfsdmFlt3awsr {
        &self.dfsdm_flt3awsr
    }
    #[doc = "0x2ac - analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn dfsdm_flt3awcfr(&self) -> &DfsdmFlt3awcfr {
        &self.dfsdm_flt3awcfr
    }
    #[doc = "0x2b0 - Extremes detector maximum register"]
    #[inline(always)]
    pub const fn dfsdm_flt3exmax(&self) -> &DfsdmFlt3exmax {
        &self.dfsdm_flt3exmax
    }
    #[doc = "0x2b4 - Extremes detector minimum register"]
    #[inline(always)]
    pub const fn dfsdm_flt3exmin(&self) -> &DfsdmFlt3exmin {
        &self.dfsdm_flt3exmin
    }
    #[doc = "0x2b8 - conversion timer register"]
    #[inline(always)]
    pub const fn dfsdm_flt3cnvtimr(&self) -> &DfsdmFlt3cnvtimr {
        &self.dfsdm_flt3cnvtimr
    }
}
#[doc = "CH0CFGR1 (rw) register accessor: channel configuration y register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cfgr1`] module"]
#[doc(alias = "CH0CFGR1")]
pub type Ch0cfgr1 = crate::Reg<ch0cfgr1::Ch0cfgr1Spec>;
#[doc = "channel configuration y register"]
pub mod ch0cfgr1;
#[doc = "CH0CFGR2 (rw) register accessor: channel configuration y register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cfgr2`] module"]
#[doc(alias = "CH0CFGR2")]
pub type Ch0cfgr2 = crate::Reg<ch0cfgr2::Ch0cfgr2Spec>;
#[doc = "channel configuration y register"]
pub mod ch0cfgr2;
#[doc = "CH0AWSCDR (rw) register accessor: analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0awscdr`] module"]
#[doc(alias = "CH0AWSCDR")]
pub type Ch0awscdr = crate::Reg<ch0awscdr::Ch0awscdrSpec>;
#[doc = "analog watchdog and short-circuit detector register"]
pub mod ch0awscdr;
#[doc = "CH0WDATR (rw) register accessor: channel watchdog filter data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0wdatr`] module"]
#[doc(alias = "CH0WDATR")]
pub type Ch0wdatr = crate::Reg<ch0wdatr::Ch0wdatrSpec>;
#[doc = "channel watchdog filter data register"]
pub mod ch0wdatr;
#[doc = "CH0DATINR (rw) register accessor: channel data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0datinr`] module"]
#[doc(alias = "CH0DATINR")]
pub type Ch0datinr = crate::Reg<ch0datinr::Ch0datinrSpec>;
#[doc = "channel data input register"]
pub mod ch0datinr;
#[doc = "CH0DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0dlyr`] module"]
#[doc(alias = "CH0DLYR")]
pub type Ch0dlyr = crate::Reg<ch0dlyr::Ch0dlyrSpec>;
#[doc = "channel y delay register"]
pub mod ch0dlyr;
#[doc = "CH1CFGR1 (rw) register accessor: CH1CFGR1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cfgr1`] module"]
#[doc(alias = "CH1CFGR1")]
pub type Ch1cfgr1 = crate::Reg<ch1cfgr1::Ch1cfgr1Spec>;
#[doc = "CH1CFGR1"]
pub mod ch1cfgr1;
#[doc = "CH1CFGR2 (rw) register accessor: CH1CFGR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cfgr2`] module"]
#[doc(alias = "CH1CFGR2")]
pub type Ch1cfgr2 = crate::Reg<ch1cfgr2::Ch1cfgr2Spec>;
#[doc = "CH1CFGR2"]
pub mod ch1cfgr2;
#[doc = "CH1AWSCDR (rw) register accessor: CH1AWSCDR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1awscdr`] module"]
#[doc(alias = "CH1AWSCDR")]
pub type Ch1awscdr = crate::Reg<ch1awscdr::Ch1awscdrSpec>;
#[doc = "CH1AWSCDR"]
pub mod ch1awscdr;
#[doc = "CH1WDATR (rw) register accessor: CH1WDATR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1wdatr`] module"]
#[doc(alias = "CH1WDATR")]
pub type Ch1wdatr = crate::Reg<ch1wdatr::Ch1wdatrSpec>;
#[doc = "CH1WDATR"]
pub mod ch1wdatr;
#[doc = "CH1DATINR (rw) register accessor: CH1DATINR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1datinr`] module"]
#[doc(alias = "CH1DATINR")]
pub type Ch1datinr = crate::Reg<ch1datinr::Ch1datinrSpec>;
#[doc = "CH1DATINR"]
pub mod ch1datinr;
#[doc = "CH1DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1dlyr`] module"]
#[doc(alias = "CH1DLYR")]
pub type Ch1dlyr = crate::Reg<ch1dlyr::Ch1dlyrSpec>;
#[doc = "channel y delay register"]
pub mod ch1dlyr;
#[doc = "CH2CFGR1 (rw) register accessor: CH2CFGR1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cfgr1`] module"]
#[doc(alias = "CH2CFGR1")]
pub type Ch2cfgr1 = crate::Reg<ch2cfgr1::Ch2cfgr1Spec>;
#[doc = "CH2CFGR1"]
pub mod ch2cfgr1;
#[doc = "CH2CFGR2 (rw) register accessor: CH2CFGR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cfgr2`] module"]
#[doc(alias = "CH2CFGR2")]
pub type Ch2cfgr2 = crate::Reg<ch2cfgr2::Ch2cfgr2Spec>;
#[doc = "CH2CFGR2"]
pub mod ch2cfgr2;
#[doc = "CH2AWSCDR (rw) register accessor: CH2AWSCDR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2awscdr`] module"]
#[doc(alias = "CH2AWSCDR")]
pub type Ch2awscdr = crate::Reg<ch2awscdr::Ch2awscdrSpec>;
#[doc = "CH2AWSCDR"]
pub mod ch2awscdr;
#[doc = "CH2WDATR (rw) register accessor: CH2WDATR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2wdatr`] module"]
#[doc(alias = "CH2WDATR")]
pub type Ch2wdatr = crate::Reg<ch2wdatr::Ch2wdatrSpec>;
#[doc = "CH2WDATR"]
pub mod ch2wdatr;
#[doc = "CH2DATINR (rw) register accessor: CH2DATINR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2datinr`] module"]
#[doc(alias = "CH2DATINR")]
pub type Ch2datinr = crate::Reg<ch2datinr::Ch2datinrSpec>;
#[doc = "CH2DATINR"]
pub mod ch2datinr;
#[doc = "CH2DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2dlyr`] module"]
#[doc(alias = "CH2DLYR")]
pub type Ch2dlyr = crate::Reg<ch2dlyr::Ch2dlyrSpec>;
#[doc = "channel y delay register"]
pub mod ch2dlyr;
#[doc = "CH3CFGR1 (rw) register accessor: CH3CFGR1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cfgr1`] module"]
#[doc(alias = "CH3CFGR1")]
pub type Ch3cfgr1 = crate::Reg<ch3cfgr1::Ch3cfgr1Spec>;
#[doc = "CH3CFGR1"]
pub mod ch3cfgr1;
#[doc = "CH3CFGR2 (rw) register accessor: CH3CFGR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cfgr2`] module"]
#[doc(alias = "CH3CFGR2")]
pub type Ch3cfgr2 = crate::Reg<ch3cfgr2::Ch3cfgr2Spec>;
#[doc = "CH3CFGR2"]
pub mod ch3cfgr2;
#[doc = "CH3AWSCDR (rw) register accessor: CH3AWSCDR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3awscdr`] module"]
#[doc(alias = "CH3AWSCDR")]
pub type Ch3awscdr = crate::Reg<ch3awscdr::Ch3awscdrSpec>;
#[doc = "CH3AWSCDR"]
pub mod ch3awscdr;
#[doc = "CH3WDATR (rw) register accessor: CH3WDATR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3wdatr`] module"]
#[doc(alias = "CH3WDATR")]
pub type Ch3wdatr = crate::Reg<ch3wdatr::Ch3wdatrSpec>;
#[doc = "CH3WDATR"]
pub mod ch3wdatr;
#[doc = "CH3DATINR (rw) register accessor: CH3DATINR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3datinr`] module"]
#[doc(alias = "CH3DATINR")]
pub type Ch3datinr = crate::Reg<ch3datinr::Ch3datinrSpec>;
#[doc = "CH3DATINR"]
pub mod ch3datinr;
#[doc = "CH3DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3dlyr`] module"]
#[doc(alias = "CH3DLYR")]
pub type Ch3dlyr = crate::Reg<ch3dlyr::Ch3dlyrSpec>;
#[doc = "channel y delay register"]
pub mod ch3dlyr;
#[doc = "CH4CFGR1 (rw) register accessor: CH4CFGR1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cfgr1`] module"]
#[doc(alias = "CH4CFGR1")]
pub type Ch4cfgr1 = crate::Reg<ch4cfgr1::Ch4cfgr1Spec>;
#[doc = "CH4CFGR1"]
pub mod ch4cfgr1;
#[doc = "CH4CFGR2 (rw) register accessor: CH4CFGR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cfgr2`] module"]
#[doc(alias = "CH4CFGR2")]
pub type Ch4cfgr2 = crate::Reg<ch4cfgr2::Ch4cfgr2Spec>;
#[doc = "CH4CFGR2"]
pub mod ch4cfgr2;
#[doc = "CH4AWSCDR (rw) register accessor: CH4AWSCDR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4awscdr`] module"]
#[doc(alias = "CH4AWSCDR")]
pub type Ch4awscdr = crate::Reg<ch4awscdr::Ch4awscdrSpec>;
#[doc = "CH4AWSCDR"]
pub mod ch4awscdr;
#[doc = "CH4WDATR (rw) register accessor: CH4WDATR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4wdatr`] module"]
#[doc(alias = "CH4WDATR")]
pub type Ch4wdatr = crate::Reg<ch4wdatr::Ch4wdatrSpec>;
#[doc = "CH4WDATR"]
pub mod ch4wdatr;
#[doc = "CH4DATINR (rw) register accessor: CH4DATINR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4datinr`] module"]
#[doc(alias = "CH4DATINR")]
pub type Ch4datinr = crate::Reg<ch4datinr::Ch4datinrSpec>;
#[doc = "CH4DATINR"]
pub mod ch4datinr;
#[doc = "CH4DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4dlyr`] module"]
#[doc(alias = "CH4DLYR")]
pub type Ch4dlyr = crate::Reg<ch4dlyr::Ch4dlyrSpec>;
#[doc = "channel y delay register"]
pub mod ch4dlyr;
#[doc = "CH5CFGR1 (rw) register accessor: CH5CFGR1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cfgr1`] module"]
#[doc(alias = "CH5CFGR1")]
pub type Ch5cfgr1 = crate::Reg<ch5cfgr1::Ch5cfgr1Spec>;
#[doc = "CH5CFGR1"]
pub mod ch5cfgr1;
#[doc = "CH5CFGR2 (rw) register accessor: CH5CFGR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cfgr2`] module"]
#[doc(alias = "CH5CFGR2")]
pub type Ch5cfgr2 = crate::Reg<ch5cfgr2::Ch5cfgr2Spec>;
#[doc = "CH5CFGR2"]
pub mod ch5cfgr2;
#[doc = "CH5AWSCDR (rw) register accessor: CH5AWSCDR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5awscdr`] module"]
#[doc(alias = "CH5AWSCDR")]
pub type Ch5awscdr = crate::Reg<ch5awscdr::Ch5awscdrSpec>;
#[doc = "CH5AWSCDR"]
pub mod ch5awscdr;
#[doc = "CH5WDATR (rw) register accessor: CH5WDATR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5wdatr`] module"]
#[doc(alias = "CH5WDATR")]
pub type Ch5wdatr = crate::Reg<ch5wdatr::Ch5wdatrSpec>;
#[doc = "CH5WDATR"]
pub mod ch5wdatr;
#[doc = "CH5DATINR (rw) register accessor: CH5DATINR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5datinr`] module"]
#[doc(alias = "CH5DATINR")]
pub type Ch5datinr = crate::Reg<ch5datinr::Ch5datinrSpec>;
#[doc = "CH5DATINR"]
pub mod ch5datinr;
#[doc = "CH5DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5dlyr`] module"]
#[doc(alias = "CH5DLYR")]
pub type Ch5dlyr = crate::Reg<ch5dlyr::Ch5dlyrSpec>;
#[doc = "channel y delay register"]
pub mod ch5dlyr;
#[doc = "CH6CFGR1 (rw) register accessor: CH6CFGR1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cfgr1`] module"]
#[doc(alias = "CH6CFGR1")]
pub type Ch6cfgr1 = crate::Reg<ch6cfgr1::Ch6cfgr1Spec>;
#[doc = "CH6CFGR1"]
pub mod ch6cfgr1;
#[doc = "CH6CFGR2 (rw) register accessor: CH6CFGR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cfgr2`] module"]
#[doc(alias = "CH6CFGR2")]
pub type Ch6cfgr2 = crate::Reg<ch6cfgr2::Ch6cfgr2Spec>;
#[doc = "CH6CFGR2"]
pub mod ch6cfgr2;
#[doc = "CH6AWSCDR (rw) register accessor: CH6AWSCDR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6awscdr`] module"]
#[doc(alias = "CH6AWSCDR")]
pub type Ch6awscdr = crate::Reg<ch6awscdr::Ch6awscdrSpec>;
#[doc = "CH6AWSCDR"]
pub mod ch6awscdr;
#[doc = "CH6WDATR (rw) register accessor: CH6WDATR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6wdatr`] module"]
#[doc(alias = "CH6WDATR")]
pub type Ch6wdatr = crate::Reg<ch6wdatr::Ch6wdatrSpec>;
#[doc = "CH6WDATR"]
pub mod ch6wdatr;
#[doc = "CH6DATINR (rw) register accessor: CH6DATINR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6datinr`] module"]
#[doc(alias = "CH6DATINR")]
pub type Ch6datinr = crate::Reg<ch6datinr::Ch6datinrSpec>;
#[doc = "CH6DATINR"]
pub mod ch6datinr;
#[doc = "CH6DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6dlyr`] module"]
#[doc(alias = "CH6DLYR")]
pub type Ch6dlyr = crate::Reg<ch6dlyr::Ch6dlyrSpec>;
#[doc = "channel y delay register"]
pub mod ch6dlyr;
#[doc = "CH7CFGR1 (rw) register accessor: CH7CFGR1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cfgr1`] module"]
#[doc(alias = "CH7CFGR1")]
pub type Ch7cfgr1 = crate::Reg<ch7cfgr1::Ch7cfgr1Spec>;
#[doc = "CH7CFGR1"]
pub mod ch7cfgr1;
#[doc = "CH7CFGR2 (rw) register accessor: CH7CFGR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cfgr2`] module"]
#[doc(alias = "CH7CFGR2")]
pub type Ch7cfgr2 = crate::Reg<ch7cfgr2::Ch7cfgr2Spec>;
#[doc = "CH7CFGR2"]
pub mod ch7cfgr2;
#[doc = "CH7AWSCDR (rw) register accessor: CH7AWSCDR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7awscdr`] module"]
#[doc(alias = "CH7AWSCDR")]
pub type Ch7awscdr = crate::Reg<ch7awscdr::Ch7awscdrSpec>;
#[doc = "CH7AWSCDR"]
pub mod ch7awscdr;
#[doc = "CH7WDATR (rw) register accessor: CH7WDATR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7wdatr`] module"]
#[doc(alias = "CH7WDATR")]
pub type Ch7wdatr = crate::Reg<ch7wdatr::Ch7wdatrSpec>;
#[doc = "CH7WDATR"]
pub mod ch7wdatr;
#[doc = "CH7DATINR (rw) register accessor: CH7DATINR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7datinr`] module"]
#[doc(alias = "CH7DATINR")]
pub type Ch7datinr = crate::Reg<ch7datinr::Ch7datinrSpec>;
#[doc = "CH7DATINR"]
pub mod ch7datinr;
#[doc = "CH7DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7dlyr`] module"]
#[doc(alias = "CH7DLYR")]
pub type Ch7dlyr = crate::Reg<ch7dlyr::Ch7dlyrSpec>;
#[doc = "channel y delay register"]
pub mod ch7dlyr;
#[doc = "DFSDM_FLT0CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0cr1`] module"]
#[doc(alias = "DFSDM_FLT0CR1")]
pub type DfsdmFlt0cr1 = crate::Reg<dfsdm_flt0cr1::DfsdmFlt0cr1Spec>;
#[doc = "control register 1"]
pub mod dfsdm_flt0cr1;
#[doc = "DFSDM_FLT0CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0cr2`] module"]
#[doc(alias = "DFSDM_FLT0CR2")]
pub type DfsdmFlt0cr2 = crate::Reg<dfsdm_flt0cr2::DfsdmFlt0cr2Spec>;
#[doc = "control register 2"]
pub mod dfsdm_flt0cr2;
#[doc = "DFSDM_FLT0ISR (r) register accessor: interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0isr`] module"]
#[doc(alias = "DFSDM_FLT0ISR")]
pub type DfsdmFlt0isr = crate::Reg<dfsdm_flt0isr::DfsdmFlt0isrSpec>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt0isr;
#[doc = "DFSDM_FLT0ICR (rw) register accessor: interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0icr`] module"]
#[doc(alias = "DFSDM_FLT0ICR")]
pub type DfsdmFlt0icr = crate::Reg<dfsdm_flt0icr::DfsdmFlt0icrSpec>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt0icr;
#[doc = "DFSDM_FLT0JCHGR (rw) register accessor: injected channel group selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0jchgr`] module"]
#[doc(alias = "DFSDM_FLT0JCHGR")]
pub type DfsdmFlt0jchgr = crate::Reg<dfsdm_flt0jchgr::DfsdmFlt0jchgrSpec>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt0jchgr;
#[doc = "DFSDM_FLT0FCR (rw) register accessor: filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0fcr`] module"]
#[doc(alias = "DFSDM_FLT0FCR")]
pub type DfsdmFlt0fcr = crate::Reg<dfsdm_flt0fcr::DfsdmFlt0fcrSpec>;
#[doc = "filter control register"]
pub mod dfsdm_flt0fcr;
#[doc = "DFSDM_FLT0JDATAR (r) register accessor: data register for injected group\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0jdatar`] module"]
#[doc(alias = "DFSDM_FLT0JDATAR")]
pub type DfsdmFlt0jdatar = crate::Reg<dfsdm_flt0jdatar::DfsdmFlt0jdatarSpec>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt0jdatar;
#[doc = "DFSDM_FLT0RDATAR (r) register accessor: data register for the regular channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0rdatar`] module"]
#[doc(alias = "DFSDM_FLT0RDATAR")]
pub type DfsdmFlt0rdatar = crate::Reg<dfsdm_flt0rdatar::DfsdmFlt0rdatarSpec>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt0rdatar;
#[doc = "DFSDM_FLT0AWHTR (rw) register accessor: analog watchdog high threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0awhtr`] module"]
#[doc(alias = "DFSDM_FLT0AWHTR")]
pub type DfsdmFlt0awhtr = crate::Reg<dfsdm_flt0awhtr::DfsdmFlt0awhtrSpec>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt0awhtr;
#[doc = "DFSDM_FLT0AWLTR (rw) register accessor: analog watchdog low threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0awltr`] module"]
#[doc(alias = "DFSDM_FLT0AWLTR")]
pub type DfsdmFlt0awltr = crate::Reg<dfsdm_flt0awltr::DfsdmFlt0awltrSpec>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt0awltr;
#[doc = "DFSDM_FLT0AWSR (r) register accessor: analog watchdog status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0awsr`] module"]
#[doc(alias = "DFSDM_FLT0AWSR")]
pub type DfsdmFlt0awsr = crate::Reg<dfsdm_flt0awsr::DfsdmFlt0awsrSpec>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt0awsr;
#[doc = "DFSDM_FLT0AWCFR (rw) register accessor: analog watchdog clear flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0awcfr`] module"]
#[doc(alias = "DFSDM_FLT0AWCFR")]
pub type DfsdmFlt0awcfr = crate::Reg<dfsdm_flt0awcfr::DfsdmFlt0awcfrSpec>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt0awcfr;
#[doc = "DFSDM_FLT0EXMAX (r) register accessor: Extremes detector maximum register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0exmax`] module"]
#[doc(alias = "DFSDM_FLT0EXMAX")]
pub type DfsdmFlt0exmax = crate::Reg<dfsdm_flt0exmax::DfsdmFlt0exmaxSpec>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt0exmax;
#[doc = "DFSDM_FLT0EXMIN (r) register accessor: Extremes detector minimum register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0exmin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0exmin`] module"]
#[doc(alias = "DFSDM_FLT0EXMIN")]
pub type DfsdmFlt0exmin = crate::Reg<dfsdm_flt0exmin::DfsdmFlt0exminSpec>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt0exmin;
#[doc = "DFSDM_FLT0CNVTIMR (r) register accessor: conversion timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0cnvtimr`] module"]
#[doc(alias = "DFSDM_FLT0CNVTIMR")]
pub type DfsdmFlt0cnvtimr = crate::Reg<dfsdm_flt0cnvtimr::DfsdmFlt0cnvtimrSpec>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt0cnvtimr;
#[doc = "DFSDM_FLT1CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1cr1`] module"]
#[doc(alias = "DFSDM_FLT1CR1")]
pub type DfsdmFlt1cr1 = crate::Reg<dfsdm_flt1cr1::DfsdmFlt1cr1Spec>;
#[doc = "control register 1"]
pub mod dfsdm_flt1cr1;
#[doc = "DFSDM_FLT1CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1cr2`] module"]
#[doc(alias = "DFSDM_FLT1CR2")]
pub type DfsdmFlt1cr2 = crate::Reg<dfsdm_flt1cr2::DfsdmFlt1cr2Spec>;
#[doc = "control register 2"]
pub mod dfsdm_flt1cr2;
#[doc = "DFSDM_FLT1ISR (r) register accessor: interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1isr`] module"]
#[doc(alias = "DFSDM_FLT1ISR")]
pub type DfsdmFlt1isr = crate::Reg<dfsdm_flt1isr::DfsdmFlt1isrSpec>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt1isr;
#[doc = "DFSDM_FLT1ICR (rw) register accessor: interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1icr`] module"]
#[doc(alias = "DFSDM_FLT1ICR")]
pub type DfsdmFlt1icr = crate::Reg<dfsdm_flt1icr::DfsdmFlt1icrSpec>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt1icr;
#[doc = "DFSDM_FLT1CHGR (rw) register accessor: injected channel group selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1chgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1chgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1chgr`] module"]
#[doc(alias = "DFSDM_FLT1CHGR")]
pub type DfsdmFlt1chgr = crate::Reg<dfsdm_flt1chgr::DfsdmFlt1chgrSpec>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt1chgr;
#[doc = "DFSDM_FLT1FCR (rw) register accessor: filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1fcr`] module"]
#[doc(alias = "DFSDM_FLT1FCR")]
pub type DfsdmFlt1fcr = crate::Reg<dfsdm_flt1fcr::DfsdmFlt1fcrSpec>;
#[doc = "filter control register"]
pub mod dfsdm_flt1fcr;
#[doc = "DFSDM_FLT1JDATAR (r) register accessor: data register for injected group\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1jdatar`] module"]
#[doc(alias = "DFSDM_FLT1JDATAR")]
pub type DfsdmFlt1jdatar = crate::Reg<dfsdm_flt1jdatar::DfsdmFlt1jdatarSpec>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt1jdatar;
#[doc = "DFSDM_FLT1RDATAR (r) register accessor: data register for the regular channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1rdatar`] module"]
#[doc(alias = "DFSDM_FLT1RDATAR")]
pub type DfsdmFlt1rdatar = crate::Reg<dfsdm_flt1rdatar::DfsdmFlt1rdatarSpec>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt1rdatar;
#[doc = "DFSDM_FLT1AWHTR (rw) register accessor: analog watchdog high threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1awhtr`] module"]
#[doc(alias = "DFSDM_FLT1AWHTR")]
pub type DfsdmFlt1awhtr = crate::Reg<dfsdm_flt1awhtr::DfsdmFlt1awhtrSpec>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt1awhtr;
#[doc = "DFSDM_FLT1AWLTR (rw) register accessor: analog watchdog low threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1awltr`] module"]
#[doc(alias = "DFSDM_FLT1AWLTR")]
pub type DfsdmFlt1awltr = crate::Reg<dfsdm_flt1awltr::DfsdmFlt1awltrSpec>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt1awltr;
#[doc = "DFSDM_FLT1AWSR (r) register accessor: analog watchdog status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1awsr`] module"]
#[doc(alias = "DFSDM_FLT1AWSR")]
pub type DfsdmFlt1awsr = crate::Reg<dfsdm_flt1awsr::DfsdmFlt1awsrSpec>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt1awsr;
#[doc = "DFSDM_FLT1AWCFR (rw) register accessor: analog watchdog clear flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1awcfr`] module"]
#[doc(alias = "DFSDM_FLT1AWCFR")]
pub type DfsdmFlt1awcfr = crate::Reg<dfsdm_flt1awcfr::DfsdmFlt1awcfrSpec>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt1awcfr;
#[doc = "DFSDM_FLT1EXMAX (r) register accessor: Extremes detector maximum register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1exmax`] module"]
#[doc(alias = "DFSDM_FLT1EXMAX")]
pub type DfsdmFlt1exmax = crate::Reg<dfsdm_flt1exmax::DfsdmFlt1exmaxSpec>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt1exmax;
#[doc = "DFSDM_FLT1EXMIN (r) register accessor: Extremes detector minimum register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1exmin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1exmin`] module"]
#[doc(alias = "DFSDM_FLT1EXMIN")]
pub type DfsdmFlt1exmin = crate::Reg<dfsdm_flt1exmin::DfsdmFlt1exminSpec>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt1exmin;
#[doc = "DFSDM_FLT1CNVTIMR (r) register accessor: conversion timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1cnvtimr`] module"]
#[doc(alias = "DFSDM_FLT1CNVTIMR")]
pub type DfsdmFlt1cnvtimr = crate::Reg<dfsdm_flt1cnvtimr::DfsdmFlt1cnvtimrSpec>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt1cnvtimr;
#[doc = "DFSDM_FLT2CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2cr1`] module"]
#[doc(alias = "DFSDM_FLT2CR1")]
pub type DfsdmFlt2cr1 = crate::Reg<dfsdm_flt2cr1::DfsdmFlt2cr1Spec>;
#[doc = "control register 1"]
pub mod dfsdm_flt2cr1;
#[doc = "DFSDM_FLT2CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2cr2`] module"]
#[doc(alias = "DFSDM_FLT2CR2")]
pub type DfsdmFlt2cr2 = crate::Reg<dfsdm_flt2cr2::DfsdmFlt2cr2Spec>;
#[doc = "control register 2"]
pub mod dfsdm_flt2cr2;
#[doc = "DFSDM_FLT2ISR (r) register accessor: interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2isr`] module"]
#[doc(alias = "DFSDM_FLT2ISR")]
pub type DfsdmFlt2isr = crate::Reg<dfsdm_flt2isr::DfsdmFlt2isrSpec>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt2isr;
#[doc = "DFSDM_FLT2ICR (rw) register accessor: interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2icr`] module"]
#[doc(alias = "DFSDM_FLT2ICR")]
pub type DfsdmFlt2icr = crate::Reg<dfsdm_flt2icr::DfsdmFlt2icrSpec>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt2icr;
#[doc = "DFSDM_FLT2JCHGR (rw) register accessor: injected channel group selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2jchgr`] module"]
#[doc(alias = "DFSDM_FLT2JCHGR")]
pub type DfsdmFlt2jchgr = crate::Reg<dfsdm_flt2jchgr::DfsdmFlt2jchgrSpec>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt2jchgr;
#[doc = "DFSDM_FLT2FCR (rw) register accessor: filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2fcr`] module"]
#[doc(alias = "DFSDM_FLT2FCR")]
pub type DfsdmFlt2fcr = crate::Reg<dfsdm_flt2fcr::DfsdmFlt2fcrSpec>;
#[doc = "filter control register"]
pub mod dfsdm_flt2fcr;
#[doc = "DFSDM_FLT2JDATAR (r) register accessor: data register for injected group\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2jdatar`] module"]
#[doc(alias = "DFSDM_FLT2JDATAR")]
pub type DfsdmFlt2jdatar = crate::Reg<dfsdm_flt2jdatar::DfsdmFlt2jdatarSpec>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt2jdatar;
#[doc = "DFSDM_FLT2RDATAR (r) register accessor: data register for the regular channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2rdatar`] module"]
#[doc(alias = "DFSDM_FLT2RDATAR")]
pub type DfsdmFlt2rdatar = crate::Reg<dfsdm_flt2rdatar::DfsdmFlt2rdatarSpec>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt2rdatar;
#[doc = "DFSDM_FLT2AWHTR (rw) register accessor: analog watchdog high threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2awhtr`] module"]
#[doc(alias = "DFSDM_FLT2AWHTR")]
pub type DfsdmFlt2awhtr = crate::Reg<dfsdm_flt2awhtr::DfsdmFlt2awhtrSpec>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt2awhtr;
#[doc = "DFSDM_FLT2AWLTR (rw) register accessor: analog watchdog low threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2awltr`] module"]
#[doc(alias = "DFSDM_FLT2AWLTR")]
pub type DfsdmFlt2awltr = crate::Reg<dfsdm_flt2awltr::DfsdmFlt2awltrSpec>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt2awltr;
#[doc = "DFSDM_FLT2AWSR (r) register accessor: analog watchdog status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2awsr`] module"]
#[doc(alias = "DFSDM_FLT2AWSR")]
pub type DfsdmFlt2awsr = crate::Reg<dfsdm_flt2awsr::DfsdmFlt2awsrSpec>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt2awsr;
#[doc = "DFSDM_FLT2AWCFR (rw) register accessor: analog watchdog clear flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2awcfr`] module"]
#[doc(alias = "DFSDM_FLT2AWCFR")]
pub type DfsdmFlt2awcfr = crate::Reg<dfsdm_flt2awcfr::DfsdmFlt2awcfrSpec>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt2awcfr;
#[doc = "DFSDM_FLT2EXMAX (r) register accessor: Extremes detector maximum register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2exmax`] module"]
#[doc(alias = "DFSDM_FLT2EXMAX")]
pub type DfsdmFlt2exmax = crate::Reg<dfsdm_flt2exmax::DfsdmFlt2exmaxSpec>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt2exmax;
#[doc = "DFSDM_FLT2EXMIN (r) register accessor: Extremes detector minimum register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2exmin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2exmin`] module"]
#[doc(alias = "DFSDM_FLT2EXMIN")]
pub type DfsdmFlt2exmin = crate::Reg<dfsdm_flt2exmin::DfsdmFlt2exminSpec>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt2exmin;
#[doc = "DFSDM_FLT2CNVTIMR (r) register accessor: conversion timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2cnvtimr`] module"]
#[doc(alias = "DFSDM_FLT2CNVTIMR")]
pub type DfsdmFlt2cnvtimr = crate::Reg<dfsdm_flt2cnvtimr::DfsdmFlt2cnvtimrSpec>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt2cnvtimr;
#[doc = "DFSDM_FLT3CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3cr1`] module"]
#[doc(alias = "DFSDM_FLT3CR1")]
pub type DfsdmFlt3cr1 = crate::Reg<dfsdm_flt3cr1::DfsdmFlt3cr1Spec>;
#[doc = "control register 1"]
pub mod dfsdm_flt3cr1;
#[doc = "DFSDM_FLT3CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3cr2`] module"]
#[doc(alias = "DFSDM_FLT3CR2")]
pub type DfsdmFlt3cr2 = crate::Reg<dfsdm_flt3cr2::DfsdmFlt3cr2Spec>;
#[doc = "control register 2"]
pub mod dfsdm_flt3cr2;
#[doc = "DFSDM_FLT3ISR (r) register accessor: interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3isr`] module"]
#[doc(alias = "DFSDM_FLT3ISR")]
pub type DfsdmFlt3isr = crate::Reg<dfsdm_flt3isr::DfsdmFlt3isrSpec>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt3isr;
#[doc = "DFSDM_FLT3ICR (rw) register accessor: interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3icr`] module"]
#[doc(alias = "DFSDM_FLT3ICR")]
pub type DfsdmFlt3icr = crate::Reg<dfsdm_flt3icr::DfsdmFlt3icrSpec>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt3icr;
#[doc = "DFSDM_FLT3JCHGR (rw) register accessor: injected channel group selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3jchgr`] module"]
#[doc(alias = "DFSDM_FLT3JCHGR")]
pub type DfsdmFlt3jchgr = crate::Reg<dfsdm_flt3jchgr::DfsdmFlt3jchgrSpec>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt3jchgr;
#[doc = "DFSDM_FLT3FCR (rw) register accessor: filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3fcr`] module"]
#[doc(alias = "DFSDM_FLT3FCR")]
pub type DfsdmFlt3fcr = crate::Reg<dfsdm_flt3fcr::DfsdmFlt3fcrSpec>;
#[doc = "filter control register"]
pub mod dfsdm_flt3fcr;
#[doc = "DFSDM_FLT3JDATAR (r) register accessor: data register for injected group\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3jdatar`] module"]
#[doc(alias = "DFSDM_FLT3JDATAR")]
pub type DfsdmFlt3jdatar = crate::Reg<dfsdm_flt3jdatar::DfsdmFlt3jdatarSpec>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt3jdatar;
#[doc = "DFSDM_FLT3RDATAR (r) register accessor: data register for the regular channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3rdatar`] module"]
#[doc(alias = "DFSDM_FLT3RDATAR")]
pub type DfsdmFlt3rdatar = crate::Reg<dfsdm_flt3rdatar::DfsdmFlt3rdatarSpec>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt3rdatar;
#[doc = "DFSDM_FLT3AWHTR (rw) register accessor: analog watchdog high threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3awhtr`] module"]
#[doc(alias = "DFSDM_FLT3AWHTR")]
pub type DfsdmFlt3awhtr = crate::Reg<dfsdm_flt3awhtr::DfsdmFlt3awhtrSpec>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt3awhtr;
#[doc = "DFSDM_FLT3AWLTR (rw) register accessor: analog watchdog low threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3awltr`] module"]
#[doc(alias = "DFSDM_FLT3AWLTR")]
pub type DfsdmFlt3awltr = crate::Reg<dfsdm_flt3awltr::DfsdmFlt3awltrSpec>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt3awltr;
#[doc = "DFSDM_FLT3AWSR (r) register accessor: analog watchdog status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3awsr`] module"]
#[doc(alias = "DFSDM_FLT3AWSR")]
pub type DfsdmFlt3awsr = crate::Reg<dfsdm_flt3awsr::DfsdmFlt3awsrSpec>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt3awsr;
#[doc = "DFSDM_FLT3AWCFR (rw) register accessor: analog watchdog clear flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3awcfr`] module"]
#[doc(alias = "DFSDM_FLT3AWCFR")]
pub type DfsdmFlt3awcfr = crate::Reg<dfsdm_flt3awcfr::DfsdmFlt3awcfrSpec>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt3awcfr;
#[doc = "DFSDM_FLT3EXMAX (r) register accessor: Extremes detector maximum register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3exmax`] module"]
#[doc(alias = "DFSDM_FLT3EXMAX")]
pub type DfsdmFlt3exmax = crate::Reg<dfsdm_flt3exmax::DfsdmFlt3exmaxSpec>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt3exmax;
#[doc = "DFSDM_FLT3EXMIN (r) register accessor: Extremes detector minimum register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3exmin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3exmin`] module"]
#[doc(alias = "DFSDM_FLT3EXMIN")]
pub type DfsdmFlt3exmin = crate::Reg<dfsdm_flt3exmin::DfsdmFlt3exminSpec>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt3exmin;
#[doc = "DFSDM_FLT3CNVTIMR (r) register accessor: conversion timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3cnvtimr`] module"]
#[doc(alias = "DFSDM_FLT3CNVTIMR")]
pub type DfsdmFlt3cnvtimr = crate::Reg<dfsdm_flt3cnvtimr::DfsdmFlt3cnvtimrSpec>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt3cnvtimr;
