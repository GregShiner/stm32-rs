#[doc = "Register `CONFR2` reader"]
pub type R = crate::R<Confr2Spec>;
#[doc = "Register `CONFR2` writer"]
pub type W = crate::W<Confr2Spec>;
#[doc = "Field `NMCU` reader - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
pub type NmcuR = crate::FieldReader<u32>;
#[doc = "Field `NMCU` writer - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
pub type NmcuW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
    #[inline(always)]
    pub fn nmcu(&self) -> NmcuR {
        NmcuR::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
    #[inline(always)]
    pub fn nmcu(&mut self) -> NmcuW<Confr2Spec> {
        NmcuW::new(self, 0)
    }
}
#[doc = "JPEG codec configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`confr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Confr2Spec;
impl crate::RegisterSpec for Confr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confr2::R`](R) reader structure"]
impl crate::Readable for Confr2Spec {}
#[doc = "`write(|w| ..)` method takes [`confr2::W`](W) writer structure"]
impl crate::Writable for Confr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFR2 to value 0"]
impl crate::Resettable for Confr2Spec {}
