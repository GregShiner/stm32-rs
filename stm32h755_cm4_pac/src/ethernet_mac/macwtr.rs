#[doc = "Register `MACWTR` reader"]
pub type R = crate::R<MacwtrSpec>;
#[doc = "Register `MACWTR` writer"]
pub type W = crate::W<MacwtrSpec>;
#[doc = "Field `WTO` reader - WTO"]
pub type WtoR = crate::FieldReader;
#[doc = "Field `WTO` writer - WTO"]
pub type WtoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PWE` reader - PWE"]
pub type PweR = crate::BitReader;
#[doc = "Field `PWE` writer - PWE"]
pub type PweW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - WTO"]
    #[inline(always)]
    pub fn wto(&self) -> WtoR {
        WtoR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PWE"]
    #[inline(always)]
    pub fn pwe(&self) -> PweR {
        PweR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - WTO"]
    #[inline(always)]
    pub fn wto(&mut self) -> WtoW<MacwtrSpec> {
        WtoW::new(self, 0)
    }
    #[doc = "Bit 8 - PWE"]
    #[inline(always)]
    pub fn pwe(&mut self) -> PweW<MacwtrSpec> {
        PweW::new(self, 8)
    }
}
#[doc = "Watchdog timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`macwtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macwtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacwtrSpec;
impl crate::RegisterSpec for MacwtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macwtr::R`](R) reader structure"]
impl crate::Readable for MacwtrSpec {}
#[doc = "`write(|w| ..)` method takes [`macwtr::W`](W) writer structure"]
impl crate::Writable for MacwtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACWTR to value 0"]
impl crate::Resettable for MacwtrSpec {}
