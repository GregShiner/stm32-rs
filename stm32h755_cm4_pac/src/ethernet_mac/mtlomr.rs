#[doc = "Register `MTLOMR` reader"]
pub type R = crate::R<MtlomrSpec>;
#[doc = "Register `MTLOMR` writer"]
pub type W = crate::W<MtlomrSpec>;
#[doc = "Field `DTXSTS` reader - DTXSTS"]
pub type DtxstsR = crate::BitReader;
#[doc = "Field `DTXSTS` writer - DTXSTS"]
pub type DtxstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTPRST` reader - CNTPRST"]
pub type CntprstR = crate::BitReader;
#[doc = "Field `CNTPRST` writer - CNTPRST"]
pub type CntprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTCLR` reader - CNTCLR"]
pub type CntclrR = crate::BitReader;
#[doc = "Field `CNTCLR` writer - CNTCLR"]
pub type CntclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - DTXSTS"]
    #[inline(always)]
    pub fn dtxsts(&self) -> DtxstsR {
        DtxstsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - CNTPRST"]
    #[inline(always)]
    pub fn cntprst(&self) -> CntprstR {
        CntprstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CNTCLR"]
    #[inline(always)]
    pub fn cntclr(&self) -> CntclrR {
        CntclrR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DTXSTS"]
    #[inline(always)]
    pub fn dtxsts(&mut self) -> DtxstsW<MtlomrSpec> {
        DtxstsW::new(self, 1)
    }
    #[doc = "Bit 8 - CNTPRST"]
    #[inline(always)]
    pub fn cntprst(&mut self) -> CntprstW<MtlomrSpec> {
        CntprstW::new(self, 8)
    }
    #[doc = "Bit 9 - CNTCLR"]
    #[inline(always)]
    pub fn cntclr(&mut self) -> CntclrW<MtlomrSpec> {
        CntclrW::new(self, 9)
    }
}
#[doc = "Operating mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlomr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlomr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtlomrSpec;
impl crate::RegisterSpec for MtlomrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlomr::R`](R) reader structure"]
impl crate::Readable for MtlomrSpec {}
#[doc = "`write(|w| ..)` method takes [`mtlomr::W`](W) writer structure"]
impl crate::Writable for MtlomrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTLOMR to value 0"]
impl crate::Resettable for MtlomrSpec {}
