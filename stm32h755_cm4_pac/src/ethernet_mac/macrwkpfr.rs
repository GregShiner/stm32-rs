#[doc = "Register `MACRWKPFR` reader"]
pub type R = crate::R<MacrwkpfrSpec>;
#[doc = "Register `MACRWKPFR` writer"]
pub type W = crate::W<MacrwkpfrSpec>;
#[doc = "Field `WKUPFRMFTR` reader - WKUPFRMFTR"]
pub type WkupfrmftrR = crate::FieldReader<u32>;
#[doc = "Field `WKUPFRMFTR` writer - WKUPFRMFTR"]
pub type WkupfrmftrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WKUPFRMFTR"]
    #[inline(always)]
    pub fn wkupfrmftr(&self) -> WkupfrmftrR {
        WkupfrmftrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WKUPFRMFTR"]
    #[inline(always)]
    pub fn wkupfrmftr(&mut self) -> WkupfrmftrW<MacrwkpfrSpec> {
        WkupfrmftrW::new(self, 0)
    }
}
#[doc = "Remove wakeup packet filter register\n\nYou can [`read`](crate::Reg::read) this register and get [`macrwkpfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwkpfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacrwkpfrSpec;
impl crate::RegisterSpec for MacrwkpfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrwkpfr::R`](R) reader structure"]
impl crate::Readable for MacrwkpfrSpec {}
#[doc = "`write(|w| ..)` method takes [`macrwkpfr::W`](W) writer structure"]
impl crate::Writable for MacrwkpfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACRWKPFR to value 0"]
impl crate::Resettable for MacrwkpfrSpec {}
