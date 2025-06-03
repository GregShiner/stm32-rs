#[doc = "Register `CH6WDATR` reader"]
pub type R = crate::R<Ch6wdatrSpec>;
#[doc = "Register `CH6WDATR` writer"]
pub type W = crate::W<Ch6wdatrSpec>;
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
    pub fn wdata(&mut self) -> WdataW<Ch6wdatrSpec> {
        WdataW::new(self, 0)
    }
}
#[doc = "CH6WDATR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6wdatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6wdatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6wdatrSpec;
impl crate::RegisterSpec for Ch6wdatrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6wdatr::R`](R) reader structure"]
impl crate::Readable for Ch6wdatrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch6wdatr::W`](W) writer structure"]
impl crate::Writable for Ch6wdatrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH6WDATR to value 0"]
impl crate::Resettable for Ch6wdatrSpec {}
