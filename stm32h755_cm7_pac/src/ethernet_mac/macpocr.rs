#[doc = "Register `MACPOCR` reader"]
pub type R = crate::R<MacpocrSpec>;
#[doc = "Register `MACPOCR` writer"]
pub type W = crate::W<MacpocrSpec>;
#[doc = "Field `PTOEN` reader - PTOEN"]
pub type PtoenR = crate::BitReader;
#[doc = "Field `PTOEN` writer - PTOEN"]
pub type PtoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCEN` reader - ASYNCEN"]
pub type AsyncenR = crate::BitReader;
#[doc = "Field `ASYNCEN` writer - ASYNCEN"]
pub type AsyncenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APDREQEN` reader - APDREQEN"]
pub type ApdreqenR = crate::BitReader;
#[doc = "Field `APDREQEN` writer - APDREQEN"]
pub type ApdreqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCTRIG` reader - ASYNCTRIG"]
pub type AsynctrigR = crate::BitReader;
#[doc = "Field `ASYNCTRIG` writer - ASYNCTRIG"]
pub type AsynctrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APDREQTRIG` reader - APDREQTRIG"]
pub type ApdreqtrigR = crate::BitReader;
#[doc = "Field `APDREQTRIG` writer - APDREQTRIG"]
pub type ApdreqtrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRRDIS` reader - DRRDIS"]
pub type DrrdisR = crate::BitReader;
#[doc = "Field `DRRDIS` writer - DRRDIS"]
pub type DrrdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DN` reader - DN"]
pub type DnR = crate::FieldReader;
#[doc = "Field `DN` writer - DN"]
pub type DnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - PTOEN"]
    #[inline(always)]
    pub fn ptoen(&self) -> PtoenR {
        PtoenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ASYNCEN"]
    #[inline(always)]
    pub fn asyncen(&self) -> AsyncenR {
        AsyncenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APDREQEN"]
    #[inline(always)]
    pub fn apdreqen(&self) -> ApdreqenR {
        ApdreqenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ASYNCTRIG"]
    #[inline(always)]
    pub fn asynctrig(&self) -> AsynctrigR {
        AsynctrigR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - APDREQTRIG"]
    #[inline(always)]
    pub fn apdreqtrig(&self) -> ApdreqtrigR {
        ApdreqtrigR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DRRDIS"]
    #[inline(always)]
    pub fn drrdis(&self) -> DrrdisR {
        DrrdisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - DN"]
    #[inline(always)]
    pub fn dn(&self) -> DnR {
        DnR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PTOEN"]
    #[inline(always)]
    pub fn ptoen(&mut self) -> PtoenW<MacpocrSpec> {
        PtoenW::new(self, 0)
    }
    #[doc = "Bit 1 - ASYNCEN"]
    #[inline(always)]
    pub fn asyncen(&mut self) -> AsyncenW<MacpocrSpec> {
        AsyncenW::new(self, 1)
    }
    #[doc = "Bit 2 - APDREQEN"]
    #[inline(always)]
    pub fn apdreqen(&mut self) -> ApdreqenW<MacpocrSpec> {
        ApdreqenW::new(self, 2)
    }
    #[doc = "Bit 4 - ASYNCTRIG"]
    #[inline(always)]
    pub fn asynctrig(&mut self) -> AsynctrigW<MacpocrSpec> {
        AsynctrigW::new(self, 4)
    }
    #[doc = "Bit 5 - APDREQTRIG"]
    #[inline(always)]
    pub fn apdreqtrig(&mut self) -> ApdreqtrigW<MacpocrSpec> {
        ApdreqtrigW::new(self, 5)
    }
    #[doc = "Bit 6 - DRRDIS"]
    #[inline(always)]
    pub fn drrdis(&mut self) -> DrrdisW<MacpocrSpec> {
        DrrdisW::new(self, 6)
    }
    #[doc = "Bits 8:15 - DN"]
    #[inline(always)]
    pub fn dn(&mut self) -> DnW<MacpocrSpec> {
        DnW::new(self, 8)
    }
}
#[doc = "PTP Offload control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macpocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacpocrSpec;
impl crate::RegisterSpec for MacpocrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpocr::R`](R) reader structure"]
impl crate::Readable for MacpocrSpec {}
#[doc = "`write(|w| ..)` method takes [`macpocr::W`](W) writer structure"]
impl crate::Writable for MacpocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACPOCR to value 0"]
impl crate::Resettable for MacpocrSpec {}
