#[doc = "Register `SDSR` reader"]
pub type R = crate::R<SdsrSpec>;
#[doc = "Field `RE` reader - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1"]
pub type ReR = crate::BitReader;
#[doc = "Field `MODES1` reader - Status Mode for Bank 1 These bits define the Status Mode of SDRAM Bank 1."]
pub type Modes1R = crate::FieldReader;
#[doc = "Field `MODES2` reader - Status Mode for Bank 2 These bits define the Status Mode of SDRAM Bank 2."]
pub type Modes2R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Status Mode for Bank 1 These bits define the Status Mode of SDRAM Bank 1."]
    #[inline(always)]
    pub fn modes1(&self) -> Modes1R {
        Modes1R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Status Mode for Bank 2 These bits define the Status Mode of SDRAM Bank 2."]
    #[inline(always)]
    pub fn modes2(&self) -> Modes2R {
        Modes2R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[doc = "SDRAM Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdsrSpec;
impl crate::RegisterSpec for SdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdsr::R`](R) reader structure"]
impl crate::Readable for SdsrSpec {}
#[doc = "`reset()` method sets SDSR to value 0"]
impl crate::Resettable for SdsrSpec {}
