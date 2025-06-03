#[doc = "Register `PCSEL` reader"]
pub type R = crate::R<PcselSpec>;
#[doc = "Register `PCSEL` writer"]
pub type W = crate::W<PcselSpec>;
#[doc = "Field `PCSEL` reader - Channel x (VINP\\[i\\]) pre selection"]
pub type PcselR = crate::FieldReader<u32>;
#[doc = "Field `PCSEL` writer - Channel x (VINP\\[i\\]) pre selection"]
pub type PcselW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Channel x (VINP\\[i\\]) pre selection"]
    #[inline(always)]
    pub fn pcsel(&self) -> PcselR {
        PcselR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Channel x (VINP\\[i\\]) pre selection"]
    #[inline(always)]
    pub fn pcsel(&mut self) -> PcselW<PcselSpec> {
        PcselW::new(self, 0)
    }
}
#[doc = "ADC pre channel selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcselSpec;
impl crate::RegisterSpec for PcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcsel::R`](R) reader structure"]
impl crate::Readable for PcselSpec {}
#[doc = "`write(|w| ..)` method takes [`pcsel::W`](W) writer structure"]
impl crate::Writable for PcselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCSEL to value 0"]
impl crate::Resettable for PcselSpec {}
