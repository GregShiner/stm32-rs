#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ALGODIR` reader - Algorithm direction"]
pub type AlgodirR = crate::BitReader;
#[doc = "Field `ALGODIR` writer - Algorithm direction"]
pub type AlgodirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGOMODE0` reader - Algorithm mode"]
pub type Algomode0R = crate::FieldReader;
#[doc = "Field `ALGOMODE0` writer - Algorithm mode"]
pub type Algomode0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DATATYPE` reader - Data type selection"]
pub type DatatypeR = crate::FieldReader;
#[doc = "Field `DATATYPE` writer - Data type selection"]
pub type DatatypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYSIZE` reader - Key size selection (AES mode only)"]
pub type KeysizeR = crate::FieldReader;
#[doc = "Field `KEYSIZE` writer - Key size selection (AES mode only)"]
pub type KeysizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FFLUSH` writer - FIFO flush"]
pub type FflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPEN` reader - Cryptographic processor enable"]
pub type CrypenR = crate::BitReader;
#[doc = "Field `CRYPEN` writer - Cryptographic processor enable"]
pub type CrypenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCM_CCMPH` reader - GCM_CCMPH"]
pub type GcmCcmphR = crate::FieldReader;
#[doc = "Field `GCM_CCMPH` writer - GCM_CCMPH"]
pub type GcmCcmphW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ALGOMODE3` reader - ALGOMODE"]
pub type Algomode3R = crate::BitReader;
#[doc = "Field `ALGOMODE3` writer - ALGOMODE"]
pub type Algomode3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Algorithm direction"]
    #[inline(always)]
    pub fn algodir(&self) -> AlgodirR {
        AlgodirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Algorithm mode"]
    #[inline(always)]
    pub fn algomode0(&self) -> Algomode0R {
        Algomode0R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&self) -> DatatypeR {
        DatatypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Key size selection (AES mode only)"]
    #[inline(always)]
    pub fn keysize(&self) -> KeysizeR {
        KeysizeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Cryptographic processor enable"]
    #[inline(always)]
    pub fn crypen(&self) -> CrypenR {
        CrypenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - GCM_CCMPH"]
    #[inline(always)]
    pub fn gcm_ccmph(&self) -> GcmCcmphR {
        GcmCcmphR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - ALGOMODE"]
    #[inline(always)]
    pub fn algomode3(&self) -> Algomode3R {
        Algomode3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Algorithm direction"]
    #[inline(always)]
    pub fn algodir(&mut self) -> AlgodirW<CrSpec> {
        AlgodirW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Algorithm mode"]
    #[inline(always)]
    pub fn algomode0(&mut self) -> Algomode0W<CrSpec> {
        Algomode0W::new(self, 3)
    }
    #[doc = "Bits 6:7 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&mut self) -> DatatypeW<CrSpec> {
        DatatypeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Key size selection (AES mode only)"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KeysizeW<CrSpec> {
        KeysizeW::new(self, 8)
    }
    #[doc = "Bit 14 - FIFO flush"]
    #[inline(always)]
    pub fn fflush(&mut self) -> FflushW<CrSpec> {
        FflushW::new(self, 14)
    }
    #[doc = "Bit 15 - Cryptographic processor enable"]
    #[inline(always)]
    pub fn crypen(&mut self) -> CrypenW<CrSpec> {
        CrypenW::new(self, 15)
    }
    #[doc = "Bits 16:17 - GCM_CCMPH"]
    #[inline(always)]
    pub fn gcm_ccmph(&mut self) -> GcmCcmphW<CrSpec> {
        GcmCcmphW::new(self, 16)
    }
    #[doc = "Bit 19 - ALGOMODE"]
    #[inline(always)]
    pub fn algomode3(&mut self) -> Algomode3W<CrSpec> {
        Algomode3W::new(self, 19)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
