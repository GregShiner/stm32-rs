#[doc = "Register `STR` reader"]
pub type R = crate::R<StrSpec>;
#[doc = "Register `STR` writer"]
pub type W = crate::W<StrSpec>;
#[doc = "Field `NBLW` reader - Number of valid bits in the last word of the message"]
pub type NblwR = crate::FieldReader;
#[doc = "Field `NBLW` writer - Number of valid bits in the last word of the message"]
pub type NblwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DCAL` writer - Digest calculation"]
pub type DcalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Number of valid bits in the last word of the message"]
    #[inline(always)]
    pub fn nblw(&self) -> NblwR {
        NblwR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of valid bits in the last word of the message"]
    #[inline(always)]
    pub fn nblw(&mut self) -> NblwW<StrSpec> {
        NblwW::new(self, 0)
    }
    #[doc = "Bit 8 - Digest calculation"]
    #[inline(always)]
    pub fn dcal(&mut self) -> DcalW<StrSpec> {
        DcalW::new(self, 8)
    }
}
#[doc = "start register\n\nYou can [`read`](crate::Reg::read) this register and get [`str::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrSpec;
impl crate::RegisterSpec for StrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`str::R`](R) reader structure"]
impl crate::Readable for StrSpec {}
#[doc = "`write(|w| ..)` method takes [`str::W`](W) writer structure"]
impl crate::Writable for StrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STR to value 0"]
impl crate::Resettable for StrSpec {}
