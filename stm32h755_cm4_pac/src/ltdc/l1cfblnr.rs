#[doc = "Register `L1CFBLNR` reader"]
pub type R = crate::R<L1cfblnrSpec>;
#[doc = "Register `L1CFBLNR` writer"]
pub type W = crate::W<L1cfblnrSpec>;
#[doc = "Field `CFBLNBR` reader - Frame Buffer Line Number"]
pub type CfblnbrR = crate::FieldReader<u16>;
#[doc = "Field `CFBLNBR` writer - Frame Buffer Line Number"]
pub type CfblnbrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Frame Buffer Line Number"]
    #[inline(always)]
    pub fn cfblnbr(&self) -> CfblnbrR {
        CfblnbrR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Buffer Line Number"]
    #[inline(always)]
    pub fn cfblnbr(&mut self) -> CfblnbrW<L1cfblnrSpec> {
        CfblnbrW::new(self, 0)
    }
}
#[doc = "Layerx ColorFrame Buffer Line Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1cfblnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfblnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1cfblnrSpec;
impl crate::RegisterSpec for L1cfblnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1cfblnr::R`](R) reader structure"]
impl crate::Readable for L1cfblnrSpec {}
#[doc = "`write(|w| ..)` method takes [`l1cfblnr::W`](W) writer structure"]
impl crate::Writable for L1cfblnrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1CFBLNR to value 0"]
impl crate::Resettable for L1cfblnrSpec {}
