#[doc = "Register `PMEM` reader"]
pub type R = crate::R<PmemSpec>;
#[doc = "Register `PMEM` writer"]
pub type W = crate::W<PmemSpec>;
#[doc = "Field `MEMSET` reader - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:"]
pub type MemsetR = crate::FieldReader;
#[doc = "Field `MEMSET` writer - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:"]
pub type MemsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMWAIT` reader - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
pub type MemwaitR = crate::FieldReader;
#[doc = "Field `MEMWAIT` writer - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
pub type MemwaitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMHOLD` reader - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:"]
pub type MemholdR = crate::FieldReader;
#[doc = "Field `MEMHOLD` writer - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:"]
pub type MemholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMHIZ` reader - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:"]
pub type MemhizR = crate::FieldReader;
#[doc = "Field `MEMHIZ` writer - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:"]
pub type MemhizW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:"]
    #[inline(always)]
    pub fn memset(&self) -> MemsetR {
        MemsetR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
    #[inline(always)]
    pub fn memwait(&self) -> MemwaitR {
        MemwaitR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:"]
    #[inline(always)]
    pub fn memhold(&self) -> MemholdR {
        MemholdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:"]
    #[inline(always)]
    pub fn memhiz(&self) -> MemhizR {
        MemhizR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:"]
    #[inline(always)]
    pub fn memset(&mut self) -> MemsetW<PmemSpec> {
        MemsetW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
    #[inline(always)]
    pub fn memwait(&mut self) -> MemwaitW<PmemSpec> {
        MemwaitW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:"]
    #[inline(always)]
    pub fn memhold(&mut self) -> MemholdW<PmemSpec> {
        MemholdW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:"]
    #[inline(always)]
    pub fn memhiz(&mut self) -> MemhizW<PmemSpec> {
        MemhizW::new(self, 24)
    }
}
#[doc = "The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access.\n\nYou can [`read`](crate::Reg::read) this register and get [`pmem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmemSpec;
impl crate::RegisterSpec for PmemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmem::R`](R) reader structure"]
impl crate::Readable for PmemSpec {}
#[doc = "`write(|w| ..)` method takes [`pmem::W`](W) writer structure"]
impl crate::Writable for PmemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMEM to value 0xfcfc_fcfc"]
impl crate::Resettable for PmemSpec {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
