#[doc = "Register `OTG_HS_HPTXFSIZ` reader"]
pub type R = crate::R<OtgHsHptxfsizSpec>;
#[doc = "Register `OTG_HS_HPTXFSIZ` writer"]
pub type W = crate::W<OtgHsHptxfsizSpec>;
#[doc = "Field `PTXSA` reader - Host periodic TxFIFO start address"]
pub type PtxsaR = crate::FieldReader<u16>;
#[doc = "Field `PTXSA` writer - Host periodic TxFIFO start address"]
pub type PtxsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTXFD` reader - Host periodic TxFIFO depth"]
pub type PtxfdR = crate::FieldReader<u16>;
#[doc = "Field `PTXFD` writer - Host periodic TxFIFO depth"]
pub type PtxfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PtxsaR {
        PtxsaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfd(&self) -> PtxfdR {
        PtxfdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&mut self) -> PtxsaW<OtgHsHptxfsizSpec> {
        PtxsaW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfd(&mut self) -> PtxfdW<OtgHsHptxfsizSpec> {
        PtxfdW::new(self, 16)
    }
}
#[doc = "OTG_HS Host periodic transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_hptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsHptxfsizSpec;
impl crate::RegisterSpec for OtgHsHptxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hptxfsiz::R`](R) reader structure"]
impl crate::Readable for OtgHsHptxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hptxfsiz::W`](W) writer structure"]
impl crate::Writable for OtgHsHptxfsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_HPTXFSIZ to value 0x0200_0600"]
impl crate::Resettable for OtgHsHptxfsizSpec {
    const RESET_VALUE: u32 = 0x0200_0600;
}
