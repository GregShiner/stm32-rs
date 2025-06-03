#[doc = "Register `MACSSIR` reader"]
pub type R = crate::R<MacssirSpec>;
#[doc = "Register `MACSSIR` writer"]
pub type W = crate::W<MacssirSpec>;
#[doc = "Field `SNSINC` reader - SNSINC"]
pub type SnsincR = crate::FieldReader;
#[doc = "Field `SNSINC` writer - SNSINC"]
pub type SnsincW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SSINC` reader - SSINC"]
pub type SsincR = crate::FieldReader;
#[doc = "Field `SSINC` writer - SSINC"]
pub type SsincW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - SNSINC"]
    #[inline(always)]
    pub fn snsinc(&self) -> SnsincR {
        SnsincR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SSINC"]
    #[inline(always)]
    pub fn ssinc(&self) -> SsincR {
        SsincR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - SNSINC"]
    #[inline(always)]
    pub fn snsinc(&mut self) -> SnsincW<MacssirSpec> {
        SnsincW::new(self, 8)
    }
    #[doc = "Bits 16:23 - SSINC"]
    #[inline(always)]
    pub fn ssinc(&mut self) -> SsincW<MacssirSpec> {
        SsincW::new(self, 16)
    }
}
#[doc = "Sub-second increment register\n\nYou can [`read`](crate::Reg::read) this register and get [`macssir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macssir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacssirSpec;
impl crate::RegisterSpec for MacssirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macssir::R`](R) reader structure"]
impl crate::Readable for MacssirSpec {}
#[doc = "`write(|w| ..)` method takes [`macssir::W`](W) writer structure"]
impl crate::Writable for MacssirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACSSIR to value 0"]
impl crate::Resettable for MacssirSpec {}
