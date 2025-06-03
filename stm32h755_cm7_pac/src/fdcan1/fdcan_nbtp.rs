#[doc = "Register `FDCAN_NBTP` reader"]
pub type R = crate::R<FdcanNbtpSpec>;
#[doc = "Register `FDCAN_NBTP` writer"]
pub type W = crate::W<FdcanNbtpSpec>;
#[doc = "Field `TSEG2` reader - Nominal Time segment after sample point"]
pub type Tseg2R = crate::FieldReader;
#[doc = "Field `TSEG2` writer - Nominal Time segment after sample point"]
pub type Tseg2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NTSEG1` reader - Nominal Time segment before sample point"]
pub type Ntseg1R = crate::FieldReader;
#[doc = "Field `NTSEG1` writer - Nominal Time segment before sample point"]
pub type Ntseg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NBRP` reader - Bit Rate Prescaler"]
pub type NbrpR = crate::FieldReader<u16>;
#[doc = "Field `NBRP` writer - Bit Rate Prescaler"]
pub type NbrpW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NSJW` reader - NSJW: Nominal (Re)Synchronization Jump Width"]
pub type NsjwR = crate::FieldReader;
#[doc = "Field `NSJW` writer - NSJW: Nominal (Re)Synchronization Jump Width"]
pub type NsjwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    pub fn tseg2(&self) -> Tseg2R {
        Tseg2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    pub fn ntseg1(&self) -> Ntseg1R {
        Ntseg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&self) -> NbrpR {
        NbrpR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - NSJW: Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&self) -> NsjwR {
        NsjwR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> Tseg2W<FdcanNbtpSpec> {
        Tseg2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    pub fn ntseg1(&mut self) -> Ntseg1W<FdcanNbtpSpec> {
        Ntseg1W::new(self, 8)
    }
    #[doc = "Bits 16:24 - Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&mut self) -> NbrpW<FdcanNbtpSpec> {
        NbrpW::new(self, 16)
    }
    #[doc = "Bits 25:31 - NSJW: Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&mut self) -> NsjwW<FdcanNbtpSpec> {
        NsjwW::new(self, 25)
    }
}
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_nbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_nbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanNbtpSpec;
impl crate::RegisterSpec for FdcanNbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_nbtp::R`](R) reader structure"]
impl crate::Readable for FdcanNbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_nbtp::W`](W) writer structure"]
impl crate::Writable for FdcanNbtpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_NBTP to value 0"]
impl crate::Resettable for FdcanNbtpSpec {}
