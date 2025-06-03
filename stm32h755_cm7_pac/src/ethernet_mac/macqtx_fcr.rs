#[doc = "Register `MACQTxFCR` reader"]
pub type R = crate::R<MacqtxFcrSpec>;
#[doc = "Register `MACQTxFCR` writer"]
pub type W = crate::W<MacqtxFcrSpec>;
#[doc = "Field `FCB_BPA` reader - FCB_BPA"]
pub type FcbBpaR = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - FCB_BPA"]
pub type FcbBpaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - TFE"]
pub type TfeR = crate::BitReader;
#[doc = "Field `TFE` writer - TFE"]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - PLT"]
pub type PltR = crate::FieldReader;
#[doc = "Field `PLT` writer - PLT"]
pub type PltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DZPQ` reader - DZPQ"]
pub type DzpqR = crate::BitReader;
#[doc = "Field `DZPQ` writer - DZPQ"]
pub type DzpqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - PT"]
pub type PtR = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - PT"]
pub type PtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - FCB_BPA"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FcbBpaR {
        FcbBpaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TFE"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - PLT"]
    #[inline(always)]
    pub fn plt(&self) -> PltR {
        PltR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - DZPQ"]
    #[inline(always)]
    pub fn dzpq(&self) -> DzpqR {
        DzpqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FCB_BPA"]
    #[inline(always)]
    pub fn fcb_bpa(&mut self) -> FcbBpaW<MacqtxFcrSpec> {
        FcbBpaW::new(self, 0)
    }
    #[doc = "Bit 1 - TFE"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TfeW<MacqtxFcrSpec> {
        TfeW::new(self, 1)
    }
    #[doc = "Bits 4:6 - PLT"]
    #[inline(always)]
    pub fn plt(&mut self) -> PltW<MacqtxFcrSpec> {
        PltW::new(self, 4)
    }
    #[doc = "Bit 7 - DZPQ"]
    #[inline(always)]
    pub fn dzpq(&mut self) -> DzpqW<MacqtxFcrSpec> {
        DzpqW::new(self, 7)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    pub fn pt(&mut self) -> PtW<MacqtxFcrSpec> {
        PtW::new(self, 16)
    }
}
#[doc = "Tx Queue flow control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macqtx_fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macqtx_fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacqtxFcrSpec;
impl crate::RegisterSpec for MacqtxFcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macqtx_fcr::R`](R) reader structure"]
impl crate::Readable for MacqtxFcrSpec {}
#[doc = "`write(|w| ..)` method takes [`macqtx_fcr::W`](W) writer structure"]
impl crate::Writable for MacqtxFcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACQTxFCR to value 0"]
impl crate::Resettable for MacqtxFcrSpec {}
