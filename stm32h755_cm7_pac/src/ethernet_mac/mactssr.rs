#[doc = "Register `MACTSSR` reader"]
pub type R = crate::R<MactssrSpec>;
#[doc = "Field `TSSOVF` reader - TSSOVF"]
pub type TssovfR = crate::BitReader;
#[doc = "Field `TSTARGT0` reader - TSTARGT0"]
pub type Tstargt0R = crate::BitReader;
#[doc = "Field `AUXTSTRIG` reader - AUXTSTRIG"]
pub type AuxtstrigR = crate::BitReader;
#[doc = "Field `TSTRGTERR0` reader - TSTRGTERR0"]
pub type Tstrgterr0R = crate::BitReader;
#[doc = "Field `TXTSSIS` reader - TXTSSIS"]
pub type TxtssisR = crate::BitReader;
#[doc = "Field `ATSSTN` reader - ATSSTN"]
pub type AtsstnR = crate::FieldReader;
#[doc = "Field `ATSSTM` reader - ATSSTM"]
pub type AtsstmR = crate::BitReader;
#[doc = "Field `ATSNS` reader - ATSNS"]
pub type AtsnsR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - TSSOVF"]
    #[inline(always)]
    pub fn tssovf(&self) -> TssovfR {
        TssovfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSTARGT0"]
    #[inline(always)]
    pub fn tstargt0(&self) -> Tstargt0R {
        Tstargt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AUXTSTRIG"]
    #[inline(always)]
    pub fn auxtstrig(&self) -> AuxtstrigR {
        AuxtstrigR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSTRGTERR0"]
    #[inline(always)]
    pub fn tstrgterr0(&self) -> Tstrgterr0R {
        Tstrgterr0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - TXTSSIS"]
    #[inline(always)]
    pub fn txtssis(&self) -> TxtssisR {
        TxtssisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - ATSSTN"]
    #[inline(always)]
    pub fn atsstn(&self) -> AtsstnR {
        AtsstnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - ATSSTM"]
    #[inline(always)]
    pub fn atsstm(&self) -> AtsstmR {
        AtsstmR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - ATSNS"]
    #[inline(always)]
    pub fn atsns(&self) -> AtsnsR {
        AtsnsR::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
#[doc = "Timestamp status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MactssrSpec;
impl crate::RegisterSpec for MactssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactssr::R`](R) reader structure"]
impl crate::Readable for MactssrSpec {}
#[doc = "`reset()` method sets MACTSSR to value 0"]
impl crate::Resettable for MactssrSpec {}
