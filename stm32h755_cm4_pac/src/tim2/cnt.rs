#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CntSpec>;
#[doc = "Field `CNT_L` reader - low counter value"]
pub type CntLR = crate::FieldReader<u16>;
#[doc = "Field `CNT_L` writer - low counter value"]
pub type CntLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_H` reader - High counter value"]
pub type CntHR = crate::FieldReader<u16>;
#[doc = "Field `CNT_H` writer - High counter value"]
pub type CntHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - low counter value"]
    #[inline(always)]
    pub fn cnt_l(&self) -> CntLR {
        CntLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High counter value"]
    #[inline(always)]
    pub fn cnt_h(&self) -> CntHR {
        CntHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - low counter value"]
    #[inline(always)]
    pub fn cnt_l(&mut self) -> CntLW<CntSpec> {
        CntLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High counter value"]
    #[inline(always)]
    pub fn cnt_h(&mut self) -> CntHW<CntSpec> {
        CntHW::new(self, 16)
    }
}
#[doc = "counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CntSpec {}
