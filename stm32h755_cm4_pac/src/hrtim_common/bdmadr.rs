#[doc = "Register `BDMADR` reader"]
pub type R = crate::R<BdmadrSpec>;
#[doc = "Register `BDMADR` writer"]
pub type W = crate::W<BdmadrSpec>;
#[doc = "Field `BDMADR` reader - Burst DMA Data register"]
pub type BdmadrR = crate::FieldReader<u32>;
#[doc = "Field `BDMADR` writer - Burst DMA Data register"]
pub type BdmadrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Burst DMA Data register"]
    #[inline(always)]
    pub fn bdmadr(&self) -> BdmadrR {
        BdmadrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Burst DMA Data register"]
    #[inline(always)]
    pub fn bdmadr(&mut self) -> BdmadrW<BdmadrSpec> {
        BdmadrW::new(self, 0)
    }
}
#[doc = "Burst DMA Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdmadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdmadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdmadrSpec;
impl crate::RegisterSpec for BdmadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdmadr::R`](R) reader structure"]
impl crate::Readable for BdmadrSpec {}
#[doc = "`write(|w| ..)` method takes [`bdmadr::W`](W) writer structure"]
impl crate::Writable for BdmadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDMADR to value 0"]
impl crate::Resettable for BdmadrSpec {}
