#[doc = "Register `MDMA_C8BRUR` reader"]
pub type R = crate::R<MdmaC8brurSpec>;
#[doc = "Register `MDMA_C8BRUR` writer"]
pub type W = crate::W<MdmaC8brurSpec>;
#[doc = "Field `SUV` reader - source adresse update value"]
pub type SuvR = crate::FieldReader<u16>;
#[doc = "Field `SUV` writer - source adresse update value"]
pub type SuvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DUV` reader - destination address update"]
pub type DuvR = crate::FieldReader<u16>;
#[doc = "Field `DUV` writer - destination address update"]
pub type DuvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - source adresse update value"]
    #[inline(always)]
    pub fn suv(&self) -> SuvR {
        SuvR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - destination address update"]
    #[inline(always)]
    pub fn duv(&self) -> DuvR {
        DuvR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - source adresse update value"]
    #[inline(always)]
    pub fn suv(&mut self) -> SuvW<MdmaC8brurSpec> {
        SuvW::new(self, 0)
    }
    #[doc = "Bits 16:31 - destination address update"]
    #[inline(always)]
    pub fn duv(&mut self) -> DuvW<MdmaC8brurSpec> {
        DuvW::new(self, 16)
    }
}
#[doc = "MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8brur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8brur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC8brurSpec;
impl crate::RegisterSpec for MdmaC8brurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c8brur::R`](R) reader structure"]
impl crate::Readable for MdmaC8brurSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma_c8brur::W`](W) writer structure"]
impl crate::Writable for MdmaC8brurSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C8BRUR to value 0"]
impl crate::Resettable for MdmaC8brurSpec {}
