#[doc = "Register `CH2WDATR` reader"]
pub type R = crate::R<Ch2wdatrSpec>;
#[doc = "Register `CH2WDATR` writer"]
pub type W = crate::W<Ch2wdatrSpec>;
#[doc = "Field `WDATA` reader - WDATA"]
pub type WdataR = crate::FieldReader<u16>;
#[doc = "Field `WDATA` writer - WDATA"]
pub type WdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WdataR {
        WdataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WdataW<Ch2wdatrSpec> {
        WdataW::new(self, 0)
    }
}
#[doc = "CH2WDATR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2wdatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2wdatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2wdatrSpec;
impl crate::RegisterSpec for Ch2wdatrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2wdatr::R`](R) reader structure"]
impl crate::Readable for Ch2wdatrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2wdatr::W`](W) writer structure"]
impl crate::Writable for Ch2wdatrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2WDATR to value 0"]
impl crate::Resettable for Ch2wdatrSpec {}
