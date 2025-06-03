#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `CRXBFF` writer - Clear receive buffer full flag"]
pub type CrxbffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTXBEF` writer - Clear transmit buffer empty flag"]
pub type CtxbefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRXBERF` writer - Clear receive CRC error flag"]
pub type CrxberfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRXOVRF` writer - Clear receive overrun error flag"]
pub type CrxovrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTXUNRF` writer - Clear transmit underrun error flag"]
pub type CtxunrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCF` writer - Clear transfer complete flag"]
pub type CtcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRF` writer - Clear slave resume flag"]
pub type CsrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRDYF` writer - Clear transceiver ready flag"]
pub type CrdyfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear receive buffer full flag"]
    #[inline(always)]
    pub fn crxbff(&mut self) -> CrxbffW<IcrSpec> {
        CrxbffW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear transmit buffer empty flag"]
    #[inline(always)]
    pub fn ctxbef(&mut self) -> CtxbefW<IcrSpec> {
        CtxbefW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear receive CRC error flag"]
    #[inline(always)]
    pub fn crxberf(&mut self) -> CrxberfW<IcrSpec> {
        CrxberfW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear receive overrun error flag"]
    #[inline(always)]
    pub fn crxovrf(&mut self) -> CrxovrfW<IcrSpec> {
        CrxovrfW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear transmit underrun error flag"]
    #[inline(always)]
    pub fn ctxunrf(&mut self) -> CtxunrfW<IcrSpec> {
        CtxunrfW::new(self, 4)
    }
    #[doc = "Bit 7 - Clear transfer complete flag"]
    #[inline(always)]
    pub fn ctcf(&mut self) -> CtcfW<IcrSpec> {
        CtcfW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear slave resume flag"]
    #[inline(always)]
    pub fn csrf(&mut self) -> CsrfW<IcrSpec> {
        CsrfW::new(self, 8)
    }
    #[doc = "Bit 11 - Clear transceiver ready flag"]
    #[inline(always)]
    pub fn crdyf(&mut self) -> CrdyfW<IcrSpec> {
        CrdyfW::new(self, 11)
    }
}
#[doc = "SWPMI Interrupt Flag Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
