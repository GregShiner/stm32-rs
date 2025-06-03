#[doc = "Register `CTR` reader"]
pub type R = crate::R<CtrSpec>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CtrSpec>;
#[doc = "Field `EN` reader - Cache enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Cache enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCACHEADDR` reader - Cacheable page index"]
pub type PcacheaddrR = crate::FieldReader<u16>;
#[doc = "Field `PCACHEADDR` writer - Cacheable page index"]
pub type PcacheaddrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:19 - Cacheable page index"]
    #[inline(always)]
    pub fn pcacheaddr(&self) -> PcacheaddrR {
        PcacheaddrR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CtrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 8:19 - Cacheable page index"]
    #[inline(always)]
    pub fn pcacheaddr(&mut self) -> PcacheaddrW<CtrSpec> {
        PcacheaddrW::new(self, 8)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrSpec;
impl crate::RegisterSpec for CtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTR to value 0x04"]
impl crate::Resettable for CtrSpec {
    const RESET_VALUE: u32 = 0x04;
}
